use cosmwasm_std::{
    attr, coin, coins, Addr, Api, Attribute, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo,
    Response, Uint128,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};

use crate::{
    error::RebalanceError,
    state::{CONFIG, FEE, INDEX_UNITS, REBALANCE, TOTAL_SUPPLY},
};

use crate::StdResult;

fn unwrap_addr(api: &dyn Api, addr: Option<String>, fallback: &Addr) -> StdResult<Addr> {
    Ok(addr
        .map(|v| api.addr_validate(&v))
        .transpose()?
        .unwrap_or_else(|| fallback.clone()))
}

fn mint_event(
    sender: impl Into<String>,
    receiver: impl Into<String>,
    refund_to: impl Into<String>,
    amount: Uint128,
) -> Vec<Attribute> {
    vec![
        attr("method", "mint"),
        attr("executor", sender),
        attr("receiver", receiver),
        attr("refund_to", refund_to),
        attr("amount", amount),
    ]
}

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    receiver: Option<String>,
    refund_to: Option<String>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.assert_not_paused(&env)?;
    if REBALANCE.may_load(deps.storage)?.is_some() {
        return Err(RebalanceError::OnRebalancing.into());
    }

    // addresses
    let receiver = unwrap_addr(deps.api, receiver, &info.sender)?;
    let refund_to = unwrap_addr(deps.api, refund_to, &info.sender)?;

    // state loader
    let fee = FEE.load(deps.storage)?;
    let index_units = INDEX_UNITS.load(deps.storage)?;
    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    // calculate
    let refund = index_units.calc_refund_amount(info.funds, amount)?;

    let mint_fee = fee.mint_fee.map(|v| (v * amount).max(Uint128::one()));
    let mint_send = amount.checked_sub(mint_fee.unwrap_or_default())?;

    // state applier
    TOTAL_SUPPLY.save(deps.storage, &(total_supply + amount))?;

    // response

    // mint 100
    let mint_msg = MsgMint {
        sender: env.contract.address.into_string(),
        amount: Some(coin(amount.u128(), &config.index_denom).into()),
    };

    // send fee to collector
    let fee_send_msg = mint_fee.map(|v| BankMsg::Send {
        to_address: fee.collector.into_string(),
        amount: coins(v.u128(), &config.index_denom),
    });

    // send 100 - fee to minter
    let mint_send_msg = BankMsg::Send {
        to_address: receiver.to_string(),
        amount: coins(mint_send.u128(), &config.index_denom),
    };
    // and send rest of funds to minter
    let refund_send_msg = BankMsg::Send {
        to_address: refund_to.to_string(),
        amount: refund,
    };

    // filter out None - if mint fee not set
    let msgs = [
        Some(mint_msg.into()),
        fee_send_msg.map(|v| v.into()),
        Some(mint_send_msg.into()),
        Some(refund_send_msg.into()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<CosmosMsg>>();

    let attrs = mint_event(info.sender, receiver, refund_to, amount);

    let resp = Response::new().add_messages(msgs).add_attributes(attrs);

    Ok(resp)
}

fn burn_event(
    sender: impl Into<String>,
    redeem_to: impl Into<String>,
    amount: Uint128,
) -> Vec<Attribute> {
    vec![
        attr("method", "burn"),
        attr("executor", sender),
        attr("redeem_to", redeem_to),
        attr("amount", amount),
    ]
}

pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    redeem_to: Option<String>,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.assert_not_paused(&env)?;
    if REBALANCE.may_load(deps.storage)?.is_some() {
        return Err(RebalanceError::OnRebalancing.into());
    }

    // addresses
    let redeem_to = unwrap_addr(deps.api, redeem_to, &info.sender)?;

    // state loader
    let config = CONFIG.load(deps.storage)?;
    let fee = FEE.load(deps.storage)?;
    let index_units = INDEX_UNITS.load(deps.storage)?;
    let total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    // calculate
    let received = cw_utils::must_pay(&info, &config.index_denom)?;
    let burn_fee = fee.burn_fee.map(|v| v * received);
    let burn_amount = received.checked_sub(burn_fee.unwrap_or_default())?;
    let mut burn_send_amount = index_units.calc_require_amount(burn_amount);
    burn_send_amount.sort_by(|a, b| a.denom.cmp(&b.denom));

    // state applier
    TOTAL_SUPPLY.save(deps.storage, &(total_supply - burn_amount))?;

    // response

    // burn 100 - fee
    let burn_msg = MsgBurn {
        sender: env.contract.address.into_string(),
        amount: Some(coin(burn_amount.u128(), &config.index_denom).into()),
    };

    // send fee to collector
    let fee_send_msg = burn_fee.map(|v| BankMsg::Send {
        to_address: fee.collector.into_string(),
        amount: coins(v.u128(), &config.index_denom),
    });

    // send (100 - fee) * units to burner
    let burn_send_msg = BankMsg::Send {
        to_address: redeem_to.to_string(),
        amount: burn_send_amount,
    };

    // filter out None - if mint fee not set
    let msgs = [
        Some(burn_msg.into()),
        fee_send_msg.map(|v| v.into()),
        Some(burn_send_msg.into()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<CosmosMsg>>();

    let attrs = burn_event(info.sender, redeem_to, burn_amount);

    let resp = Response::new().add_messages(msgs).add_attributes(attrs);

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{
        coin, coins,
        testing::{mock_dependencies_with_balances, mock_env, mock_info, MOCK_CONTRACT_ADDR},
        Addr, BankMsg, Coin, CosmosMsg, Decimal, SubMsg, Uint128,
    };
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};

    use crate::{
        execute::token::{burn_event, mint_event},
        state::{tests::StateBuilder, Config, Fee, INDEX_UNITS, TOTAL_SUPPLY},
    };

    use super::mint;

    fn assert_mint_resp_msgs(
        msgs: Vec<SubMsg>,
        mint_amount: (&str, u128),
        fee_amount: Option<(&str, u128)>,
        send_amount: (&str, u128),
        refund_amount: (&str, &[Coin]),
    ) {
        let mut expected_msgs: Vec<CosmosMsg> = vec![];

        expected_msgs.push(
            MsgMint {
                sender: mint_amount.0.to_string(),
                amount: Some(coin(mint_amount.1, "uibcx").into()),
            }
            .into(),
        );

        if let Some((fee_collector, fee_amount)) = fee_amount {
            expected_msgs.push(
                BankMsg::Send {
                    to_address: fee_collector.to_string(),
                    amount: coins(fee_amount, "uibcx"),
                }
                .into(),
            );
        }

        expected_msgs.push(
            BankMsg::Send {
                to_address: send_amount.0.to_string(),
                amount: coins(send_amount.1, "uibcx"),
            }
            .into(),
        );

        expected_msgs.push(
            BankMsg::Send {
                to_address: refund_amount.0.to_string(),
                amount: refund_amount.1.to_vec(),
            }
            .into(),
        );

        assert_eq!(
            msgs,
            expected_msgs
                .into_iter()
                .map(SubMsg::new)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_mint_with_fee() {
        let env = mock_env();
        let mut deps = mock_dependencies_with_balances(&[]);

        StateBuilder::default()
            .add_index_unit("uatom", "1.0")
            .with_config(Config {
                index_denom: "uibcx".to_string(),
                ..Default::default()
            })
            .with_fee(Fee {
                collector: Addr::unchecked("collector"),
                mint_fee: Some(Decimal::from_str("0.1").unwrap()),
                ..Default::default()
            }) // 10%
            .with_total_supply(10e6 as u128)
            .build(deps.as_mut().storage);

        let amount = 100u128.into();
        let refund = 10u128.into();
        let index_units = INDEX_UNITS.load(deps.as_ref().storage).unwrap();
        let index_funds = index_units.calc_require_amount(amount + refund);
        let refund_funds = index_units.calc_require_amount(refund);

        let minter = mock_info("minter", &index_funds);
        let recv = Some("receiver".to_string());
        let rfnd = Some("refund_to".to_string());

        let mint_resp = mint(deps.as_mut(), env, minter, amount, recv, rfnd).unwrap();
        assert_eq!(
            mint_resp.attributes,
            mint_event("minter", "receiver", "refund_to", amount)
        );
        assert_mint_resp_msgs(
            mint_resp.messages,
            (MOCK_CONTRACT_ADDR, 100),
            Some(("collector", 10)),
            ("receiver", 90),
            ("refund_to", &refund_funds),
        );

        assert_eq!(
            TOTAL_SUPPLY.load(deps.as_ref().storage).unwrap(),
            Uint128::from((10e6 as u128) + 100),
        );
    }

    #[test]
    fn test_mint_without_fee() {
        let env = mock_env();
        let mut deps = mock_dependencies_with_balances(&[]);

        StateBuilder::default()
            .add_index_unit("uatom", "1.0")
            .with_config(Config {
                index_denom: "uibcx".to_string(),
                ..Default::default()
            })
            .with_fee(Fee {
                collector: Addr::unchecked("collector"),
                ..Default::default()
            })
            .with_total_supply(10e6 as u128)
            .build(deps.as_mut().storage);

        let amount = 100u128.into();
        let refund = 10u128.into();
        let index_units = INDEX_UNITS.load(deps.as_ref().storage).unwrap();
        let index_funds = index_units.calc_require_amount(amount + refund);
        let refund_funds = index_units.calc_require_amount(refund);

        let minter = mock_info("minter", &index_funds);
        let recv = Some("receiver".to_string());
        let rfnd = Some("refund_to".to_string());

        let mint_resp = mint(deps.as_mut(), env, minter, amount, recv, rfnd).unwrap();
        assert_eq!(
            mint_resp.attributes,
            mint_event("minter", "receiver", "refund_to", amount)
        );
        assert_mint_resp_msgs(
            mint_resp.messages,
            (MOCK_CONTRACT_ADDR, 100),
            None,
            ("receiver", 100),
            ("refund_to", &refund_funds),
        );

        assert_eq!(
            TOTAL_SUPPLY.load(deps.as_ref().storage).unwrap(),
            Uint128::from((10e6 as u128) + 100),
        );
    }

    fn assert_burn_resp_msgs(
        msgs: Vec<SubMsg>,
        burn_amount: (&str, u128),
        fee_amount: Option<(&str, u128)>,
        send_amount: (&str, &[Coin]),
    ) {
        let mut expected_msgs: Vec<CosmosMsg> = vec![];

        expected_msgs.push(
            MsgBurn {
                sender: burn_amount.0.to_string(),
                amount: Some(coin(burn_amount.1, "uibcx").into()),
            }
            .into(),
        );

        if let Some((fee_collector, fee_amount)) = fee_amount {
            expected_msgs.push(
                BankMsg::Send {
                    to_address: fee_collector.to_string(),
                    amount: coins(fee_amount, "uibcx"),
                }
                .into(),
            );
        }

        expected_msgs.push(
            BankMsg::Send {
                to_address: send_amount.0.to_string(),
                amount: send_amount.1.to_vec(),
            }
            .into(),
        );

        assert_eq!(
            msgs,
            expected_msgs
                .into_iter()
                .map(SubMsg::new)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_burn_with_fee() {
        let env = mock_env();
        let mut deps = mock_dependencies_with_balances(&[]);

        StateBuilder::default()
            .add_index_unit("uatom", "1.0")
            .with_config(Config {
                index_denom: "uibcx".to_string(),
                ..Default::default()
            })
            .with_fee(Fee {
                collector: Addr::unchecked("collector"),
                burn_fee: Some(Decimal::from_str("0.2").unwrap()),
                ..Default::default()
            }) // 20%
            .with_total_supply(10e6 as u128)
            .build(deps.as_mut().storage);

        let amount: Uint128 = 100u128.into();
        let amount_deducted = 80u128.into();
        let index_units = INDEX_UNITS.load(deps.as_ref().storage).unwrap();
        let index_funds = index_units.calc_require_amount(amount_deducted);

        let burn_resp = super::burn(
            deps.as_mut(),
            env,
            mock_info("burner", &coins(amount.u128(), "uibcx")),
            Some("redeem_to".to_string()),
        )
        .unwrap();
        assert_eq!(
            burn_resp.attributes,
            burn_event("burner", "redeem_to", amount_deducted)
        );
        assert_burn_resp_msgs(
            burn_resp.messages,
            (MOCK_CONTRACT_ADDR, amount_deducted.u128()),
            Some(("collector", amount.u128() - amount_deducted.u128())),
            ("redeem_to", &index_funds),
        );

        assert_eq!(
            TOTAL_SUPPLY.load(deps.as_ref().storage).unwrap(),
            Uint128::from((10e6 as u128) - amount_deducted.u128()),
        );
    }

    #[test]
    fn test_burn_without_fee() {
        let env = mock_env();
        let mut deps = mock_dependencies_with_balances(&[]);

        StateBuilder::default()
            .add_index_unit("uatom", "1.0")
            .with_config(Config {
                index_denom: "uibcx".to_string(),
                ..Default::default()
            })
            .with_fee(Fee::default())
            .with_total_supply(10e6 as u128)
            .build(deps.as_mut().storage);

        let amount = 100u128.into();
        let index_units = INDEX_UNITS.load(deps.as_ref().storage).unwrap();
        let index_funds = index_units.calc_require_amount(amount);

        let burn_resp = super::burn(
            deps.as_mut(),
            env,
            mock_info("burner", &coins(amount.u128(), "uibcx")),
            Some("redeem_to".to_string()),
        )
        .unwrap();
        assert_eq!(
            burn_resp.attributes,
            burn_event("burner", "redeem_to", amount)
        );
        assert_burn_resp_msgs(
            burn_resp.messages,
            (MOCK_CONTRACT_ADDR, 100),
            None,
            ("redeem_to", &index_funds),
        );

        assert_eq!(
            TOTAL_SUPPLY.load(deps.as_ref().storage).unwrap(),
            Uint128::from((10e6 as u128) - 100),
        );
    }
}
