mod gov;
mod rebalance;

use cosmwasm_std::{attr, coin, Addr, BankMsg, Coin, CosmosMsg, Env, MessageInfo, Uint128};
use cosmwasm_std::{DepsMut, Response};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};

use crate::state::{Fee, FEE};
use crate::{
    error::ContractError,
    state::{assert_assets, get_redeem_amounts, PAUSED, TOKEN},
};

pub use crate::execute::gov::handle_msg as handle_gov_msg;
pub use crate::execute::rebalance::handle_msg as handle_rebalance_msg;

fn make_mint_msgs_with_fee_collection(
    fee: Fee,
    mint: Coin,
    receiver: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut msgs = vec![];

    if let Some(ratio) = fee.mint {
        let fee_amount = (mint.amount * ratio).max(Uint128::one());

        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: vec![coin(fee_amount.u128(), &mint.denom)],
            }
            .into(),
        );

        msgs.push(
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: vec![coin(
                    mint.amount.checked_sub(fee_amount)?.u128(),
                    &mint.denom,
                )],
            }
            .into(),
        );
    } else {
        msgs.push(
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: vec![mint],
            }
            .into(),
        );
    }

    Ok(msgs)
}

fn make_burn_msgs_with_fee_collection(
    fee: Fee,
    burn: Coin,
    burner: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut msgs = vec![];

    if let Some(ratio) = fee.burn {
        let fee_amount = (burn.amount * ratio).max(Uint128::one());

        msgs.push(
            BankMsg::Send {
                to_address: fee.collector.to_string(),
                amount: vec![coin(fee_amount.u128(), &burn.denom)],
            }
            .into(),
        );

        msgs.push(
            MsgBurn {
                sender: burner.to_string(),
                amount: Some(coin(burn.amount.checked_sub(fee_amount)?.u128(), &burn.denom).into()),
            }
            .into(),
        );
    } else {
        msgs.push(
            MsgBurn {
                sender: burner.to_string(),
                amount: Some(burn.into()),
            }
            .into(),
        );
    }

    Ok(msgs)
}

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    receiver: Option<String>,
    refund_to: Option<String>,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    // validate!
    let receiver = receiver
        .map(|v| deps.api.addr_validate(&v))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());
    let refund_to = refund_to
        .map(|v| deps.api.addr_validate(&v))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    let mut token = TOKEN.load(deps.storage)?;
    // TODO: reflect streaming fee
    let refund = assert_assets(deps.storage, info.funds, amount)?;

    token.total_supply = token.total_supply.checked_add(amount)?;
    TOKEN.save(deps.storage, &token)?;

    let mint_amount = coin(amount.u128(), &token.denom);
    let refund_amount = refund.into_iter().filter(|v| !v.amount.is_zero()).collect();

    let resp = Response::new()
        .add_message(MsgMint {
            sender: env.contract.address.into_string(),
            amount: Some(mint_amount.clone().into()),
        })
        .add_messages(make_mint_msgs_with_fee_collection(
            FEE.load(deps.storage)?,
            mint_amount,
            &receiver,
        )?)
        .add_message(BankMsg::Send {
            to_address: refund_to.to_string(),
            amount: refund_amount,
        })
        .add_attributes(vec![
            attr("method", "mint"),
            attr("executor", info.sender),
            attr("receiver", receiver),
            attr("amount", amount),
        ]);

    Ok(resp)
}

pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    redeem_to: Option<String>,
) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let mut token = TOKEN.load(deps.storage)?;
    let received = cw_utils::must_pay(&info, &token.denom)?;

    let fee = FEE.load(deps.storage)?;
    // TODO: reflect streaming fee
    let payback = get_redeem_amounts(
        deps.storage,
        fee.burn
            .map(|ratio| {
                let fee_amount = (received * ratio).max(Uint128::one());
                received.checked_sub(fee_amount).unwrap()
            })
            .unwrap_or(received),
    )?;

    token.total_supply = token.total_supply.checked_sub(received)?;
    TOKEN.save(deps.storage, &token)?;

    let redeemer = redeem_to
        .map(|v| deps.api.addr_validate(&v))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    let resp = Response::new()
        .add_messages(make_burn_msgs_with_fee_collection(
            fee,
            coin(received.u128(), &token.denom),
            &env.contract.address,
        )?)
        .add_message(BankMsg::Send {
            to_address: redeemer.to_string(),
            amount: payback,
        })
        .add_attributes(vec![
            attr("method", "burn"),
            attr("executor", info.sender),
            attr("amount", received),
        ]);

    Ok(resp)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies_with_balances, mock_env, mock_info},
        Addr, Decimal, Storage, SubMsg,
    };

    use super::*;
    use crate::state::{PauseInfo, Token, ASSETS, PAUSED};

    fn stop(storage: &mut dyn Storage, now: u64) {
        PAUSED
            .save(
                storage,
                &PauseInfo {
                    paused: true,
                    expires_at: Some(now + 1000),
                },
            )
            .unwrap()
    }

    fn resume(storage: &mut dyn Storage, now: u64) {
        PAUSED
            .save(
                storage,
                &PauseInfo {
                    paused: true,
                    expires_at: Some(now - 1000),
                },
            )
            .unwrap()
    }

    fn register_assets(storage: &mut dyn Storage, assets: &[(&str, &str)]) {
        for (denom, unit) in assets {
            ASSETS
                .save(
                    storage,
                    denom.to_string(),
                    &Decimal::from_str(unit).unwrap(),
                )
                .unwrap();
        }
    }

    #[test]
    fn test_mint() {
        let alice = Addr::unchecked("alice");
        let bob = Addr::unchecked("bob");

        let max = 10000000000000000;
        let balances = vec![coin(max, "uaaa"), coin(max, "ubbb"), coin(max, "uccc")];
        let mut deps = mock_dependencies_with_balances(&[
            (alice.as_str(), balances.as_slice()),
            (bob.as_str(), balances.as_slice()),
        ]);
        let env = mock_env();
        let now = env.block.time.seconds();

        FEE.save(
            deps.as_mut().storage,
            &Fee {
                collector: Addr::unchecked("test"),
                mint: Default::default(),
                burn: Default::default(),
                stream: Default::default(),
                stream_last_collected_at: Default::default(),
            },
        )
        .unwrap();

        TOKEN
            .save(
                deps.as_mut().storage,
                &Token {
                    denom: "ibcx".to_string(),
                    reserve_denom: "uosmo".to_string(),
                    total_supply: Uint128::zero(),
                },
            )
            .unwrap();
        register_assets(
            deps.as_mut().storage,
            &[("uaaa", "1.0"), ("ubbb", "1.2"), ("uccc", "1.5")],
        );

        stop(deps.as_mut().storage, now);

        assert!(matches!(
            mint(
                deps.as_mut(),
                env.clone(),
                mock_info(alice.as_str(), &[]),
                Uint128::new(10),
                Some(bob.clone().into_string()),
                Some(bob.clone().into_string()),
            )
            .unwrap_err(),
            ContractError::Paused {}
        ));

        resume(deps.as_mut().storage, now);

        let resp = mint(
            deps.as_mut(),
            env.clone(),
            mock_info(
                alice.as_str(),
                &[coin(20, "uaaa"), coin(24, "ubbb"), coin(30, "uccc")],
            ),
            Uint128::new(10),
            Some(alice.clone().into_string()),
            Some(bob.clone().into_string()),
        )
        .unwrap();

        let token = TOKEN.load(deps.as_ref().storage).unwrap();
        assert_eq!(token.total_supply, Uint128::new(10));
        assert_eq!(
            resp.messages,
            vec![
                SubMsg::new(MsgMint {
                    sender: env.contract.address.to_string(),
                    amount: Some(coin(10, "ibcx").into()),
                }),
                SubMsg::new(BankMsg::Send {
                    to_address: alice.to_string(),
                    amount: vec![coin(10, "ibcx"),],
                }),
                SubMsg::new(BankMsg::Send {
                    to_address: bob.into_string(),
                    amount: vec![coin(10, "uaaa"), coin(12, "ubbb"), coin(15, "uccc"),]
                })
            ]
        );
    }

    #[test]
    fn test_burn() {}
}
