mod fee;
mod gov;
mod rebalance;

use cosmwasm_std::{attr, coin, BankMsg, Env, MessageInfo, Uint128};
use cosmwasm_std::{DepsMut, Response};

use crate::{
    error::ContractError,
    state::{assert_assets, get_redeem_amounts, FEE, PAUSED, TOKEN},
};

pub use crate::execute::fee::collect_streaming_fee;
pub use crate::execute::gov::handle_msg as handle_gov_msg;
pub use crate::execute::rebalance::handle_msg as handle_rebalance_msg;

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
        .add_messages(fee::make_mint_msgs_with_fee_collection(
            FEE.load(deps.storage)?,
            mint_amount,
            &env.contract.address,
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
            attr("refund_to", refund_to),
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
    let deducted = fee
        .burn
        .map(|ratio| {
            let fee_amount = (received * ratio).max(Uint128::one());
            received.checked_sub(fee_amount).unwrap()
        })
        .unwrap_or(received);
    let payback = get_redeem_amounts(deps.storage, deducted)?;

    token.total_supply = token.total_supply.checked_sub(received)?;
    TOKEN.save(deps.storage, &token)?;

    let redeemer = redeem_to
        .map(|v| deps.api.addr_validate(&v))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    let resp = Response::new()
        .add_messages(fee::make_burn_msgs_with_fee_collection(
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
            attr("redeem_to", redeemer),
            attr("amount", received),
        ]);

    Ok(resp)
}

pub fn realize(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let fee = FEE.load(deps.storage)?;
    if fee.collector != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let msg = fee::realize_streaming_fee(deps.storage)?;

    Ok(Response::new().add_message(msg).add_attributes(vec![
        attr("method", "realize"),
        attr("executor", info.sender),
    ]))
}

#[cfg(test)]
mod test {

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        OwnedDeps, SubMsg,
    };

    use crate::test::{default_fee, default_token, register_assets};

    use super::*;

    fn setup(deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>) {
        PAUSED
            .save(deps.as_mut().storage, &Default::default())
            .unwrap();
        FEE.save(deps.as_mut().storage, &default_fee()).unwrap();
        TOKEN.save(deps.as_mut().storage, &default_token()).unwrap();

        register_assets(
            deps.as_mut().storage,
            &[
                ("ujpy", "1.0"),
                ("ukrw", "1.0"),
                ("ueur", "1.2"),
                ("uusd", "1.5"),
            ],
        );
    }

    mod mint {

        use super::*;

        #[test]
        fn test_mint() {
            let mut deps = mock_dependencies();

            setup(&mut deps);

            let info = mock_info(
                "minter",
                &[
                    coin(100000, "ujpy"),
                    coin(120000, "ukrw"),
                    coin(150000, "ueur"),
                    coin(200000, "uusd"),
                ],
            );
            let amount = Uint128::new(100000);
            let resp = mint(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                amount,
                Some("receiver".to_string()),
                Some("refund".to_string()),
            )
            .unwrap();

            // check attributes
            assert_eq!(
                resp.attributes,
                vec![
                    attr("method", "mint"),
                    attr("executor", info.sender.to_string()),
                    attr("receiver", "receiver"),
                    attr("refund_to", "refund"),
                    attr("amount", amount.to_string()),
                ]
            );

            // check refund amounts - removed ujpy element
            assert_eq!(
                resp.messages.last().unwrap(),
                &SubMsg::new(BankMsg::Send {
                    to_address: "refund".to_string(),
                    amount: vec![
                        coin(30000, "ueur"),
                        coin(20000, "ukrw"),
                        coin(50000, "uusd"),
                    ]
                })
            );

            // check state
            assert_eq!(
                TOKEN.load(deps.as_ref().storage).unwrap().total_supply,
                amount + default_token().total_supply
            );
        }
    }

    mod burn {
        use super::*;

        #[test]
        fn test_burn() {
            let mut deps = mock_dependencies();

            setup(&mut deps);

            let amount = Uint128::new(100000);
            let info = mock_info("burner", &[coin(amount.u128(), "uibcx")]);

            let resp = burn(
                deps.as_mut(),
                mock_env(),
                info,
                Some("redeemer".to_string()),
            )
            .unwrap();

            // check response
            assert_eq!(
                resp.attributes,
                vec![
                    attr("method", "burn"),
                    attr("executor", "burner"),
                    attr("redeem_to", "redeemer"),
                    attr("amount", amount.to_string())
                ]
            );

            // check redeemed amount
            assert_eq!(
                resp.messages.last().unwrap(),
                &SubMsg::new(BankMsg::Send {
                    to_address: "redeemer".to_string(),
                    amount: vec![
                        coin(120000, "ueur"),
                        coin(100000, "ujpy"),
                        coin(100000, "ukrw"),
                        coin(150000, "uusd"),
                    ]
                })
            );

            // check state
            assert_eq!(
                TOKEN.load(deps.as_ref().storage).unwrap().total_supply,
                Uint128::zero()
            );
        }
    }
}
