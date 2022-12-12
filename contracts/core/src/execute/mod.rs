mod gov;
mod rebalance;

use cosmwasm_std::{attr, coin, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};

use crate::{
    error::ContractError,
    state::{assert_assets, get_redeem_amounts, PAUSED, TOKEN},
};

pub use crate::execute::gov::handle_msg as handle_gov_msg;
pub use crate::execute::rebalance::handle_msg as handle_rebalance_msg;

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    receiver: Option<String>,
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

    let mut token = TOKEN.load(deps.storage)?;
    let refund = assert_assets(deps.storage, info.funds, amount)?;

    token.total_supply = token.total_supply.checked_add(amount)?;
    TOKEN.save(deps.storage, &token)?;

    let mint_amount = coin(amount.u128(), token.denom);
    let mut send_amount = refund;
    send_amount.push(mint_amount.clone());
    let send_amount = send_amount
        .into_iter()
        .filter(|v| !v.amount.is_zero())
        .collect();

    let resp = Response::new()
        .add_message(MsgMint {
            sender: env.contract.address.into_string(),
            amount: Some(mint_amount.into()),
        })
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: send_amount,
        })
        .add_attributes(vec![
            attr("method", "mint"),
            attr("executor", info.sender),
            attr("receiver", receiver),
            attr("amount", amount),
        ]);

    Ok(resp)
}

pub fn burn(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    PAUSED
        .load(deps.storage)?
        .refresh(deps.storage, &env)?
        .assert_not_paused()?;

    let mut token = TOKEN.load(deps.storage)?;
    let received = cw_utils::must_pay(&info, &token.denom)?;
    let payback = get_redeem_amounts(deps.storage, received)?;

    token.total_supply = token.total_supply.checked_sub(received)?;
    TOKEN.save(deps.storage, &token)?;

    let resp = Response::new()
        .add_message(MsgBurn {
            sender: env.contract.address.to_string(),
            amount: Some(coin(received.u128(), token.denom).into()),
        })
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
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
                Some(bob.into_string()),
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
                    amount: vec![
                        coin(10, "uaaa"),
                        coin(12, "ubbb"),
                        coin(15, "uccc"),
                        coin(10, "ibcx"),
                    ],
                }),
            ]
        );
    }

    #[test]
    fn test_burn() {}
}
