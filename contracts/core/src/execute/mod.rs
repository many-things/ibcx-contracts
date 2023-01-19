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
    minter: &Addr,
    receiver: &Addr,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut msgs = vec![MsgMint {
        sender: minter.clone().into_string(),
        amount: Some(mint.clone().into()),
    }
    .into()];

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
        .add_messages(make_mint_msgs_with_fee_collection(
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
            attr("redeem_to", redeemer),
            attr("amount", received),
        ]);

    Ok(resp)
}

#[cfg(test)]
mod test {

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
        Decimal, OwnedDeps, SubMsg,
    };

    use crate::{
        execute::make_mint_msgs_with_fee_collection,
        test::default_fee,
        test::{default_token, register_assets},
    };

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
        fn test_make_mint_msgs() {
            let expected_mint_msg = MsgMint {
                sender: "minter".to_string(),
                amount: Some(coin(100, "uibcx").into()),
            }
            .into();
            let fee_ratio = Decimal::from_ratio(10u128, 100u128);
            let mint = coin(100, "uibcx");
            let minter = Addr::unchecked("minter");
            let receiver = Addr::unchecked("receiver");

            // with mint fee
            let fee = super::Fee {
                mint: Some(fee_ratio),
                ..default_fee()
            };
            let mut msgs =
                make_mint_msgs_with_fee_collection(fee, mint.clone(), &minter, &receiver).unwrap();
            assert_eq!(msgs.remove(0), expected_mint_msg);
            assert_eq!(
                msgs,
                vec![
                    BankMsg::Send {
                        to_address: "collector".to_string(),
                        amount: vec![coin(10, "uibcx")],
                    }
                    .into(),
                    BankMsg::Send {
                        to_address: "receiver".to_string(),
                        amount: vec![coin(90, "uibcx")],
                    }
                    .into()
                ]
            );

            // with no fee
            let fee = default_fee();
            let mut msgs =
                make_mint_msgs_with_fee_collection(fee, mint, &minter, &receiver).unwrap();
            assert_eq!(msgs.remove(0), expected_mint_msg);
            assert_eq!(
                msgs,
                vec![BankMsg::Send {
                    to_address: "receiver".to_string(),
                    amount: vec![coin(100, "uibcx")],
                }
                .into()]
            );
        }

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
                amount.clone(),
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
        fn test_make_burn_msgs() {
            // with burn fee
            let fee_ratio = Decimal::from_ratio(10u128, 100u128);
            let fee = super::Fee {
                burn: Some(fee_ratio),
                ..default_fee()
            };
            let burn = coin(100, "uibcx");
            let burner = Addr::unchecked("burner");

            let expected_msgs = vec![
                BankMsg::Send {
                    to_address: fee.collector.to_string(),
                    amount: vec![coin(10, &burn.denom)],
                }
                .into(),
                MsgBurn {
                    sender: burner.to_string(),
                    amount: Some(coin(90, &burn.denom).into()),
                }
                .into(),
            ];

            let msgs = make_burn_msgs_with_fee_collection(fee, burn, &burner).unwrap();
            assert_eq!(msgs, expected_msgs);

            // with no fee
            let fee = default_fee();
            let burn = coin(100, "uibcx");
            let burner = Addr::unchecked("burner");

            let expected_msgs = vec![MsgBurn {
                sender: burner.to_string(),
                amount: Some(burn.clone().into()),
            }
            .into()];

            let msgs = make_burn_msgs_with_fee_collection(fee, burn, &burner).unwrap();
            assert_eq!(msgs, expected_msgs);
        }

        #[test]
        fn test_burn() {
            let mut deps = mock_dependencies();

            setup(&mut deps);

            let amount = Uint128::new(100000);
            let info = mock_info("burner", &[coin(amount.u128(), "uibcx")]);

            let resp = burn(
                deps.as_mut(),
                mock_env(),
                info.clone(),
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
