use crate::airdrop::{Airdrop, BearerAirdrop, OpenAirdrop};
use crate::error::ContractError;
use crate::state::{airdrops, load_airdrop};
use cosmwasm_std::{attr, Addr, Attribute, DepsMut, MessageInfo, Response};
use ibcx_interface::airdrop::{AirdropId, AirdropType};

pub fn fund(deps: DepsMut, info: MessageInfo, id: AirdropId) -> Result<Response, ContractError> {
    let (airdrop_id, airdrop) = load_airdrop(deps.storage, id)?;

    match airdrop {
        Airdrop::Open(inner) => fund_open(deps, info, airdrop_id, inner),
        Airdrop::Bearer(inner) => fund_bearer(deps, info, airdrop_id, inner),
    }
}

fn fund_event(sender: Addr, typ: AirdropType, id: u64, add: impl Into<u128>) -> Vec<Attribute> {
    vec![
        attr("action", "fund"),
        attr("executor", sender),
        attr("airdrop_type", typ),
        attr("airdrop_id", id.to_string()),
        attr("amount", add.into().to_string()),
    ]
}

fn fund_open(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    mut airdrop: OpenAirdrop,
) -> Result<Response, ContractError> {
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let additional_funds = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(additional_funds)?;

    // event attributes
    let attrs = fund_event(info.sender, AirdropType::Open, id, additional_funds);

    // apply to state
    airdrops().save(deps.storage, id, &airdrop.into())?;

    Ok(Response::new().add_attributes(attrs))
}

fn fund_bearer(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    mut airdrop: BearerAirdrop,
) -> Result<Response, ContractError> {
    if airdrop.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if airdrop.closed_at.is_some() {
        return Err(ContractError::AirdropClosed {});
    }

    let additional_funds = cw_utils::must_pay(&info, &airdrop.denom)?;
    airdrop.total_amount = airdrop.total_amount.checked_add(additional_funds)?;

    // event attributes
    let attrs = fund_event(info.sender, AirdropType::Bearer, id, additional_funds);

    // apply to state
    airdrops().save(deps.storage, id, &airdrop.into())?;

    Ok(Response::new().add_attributes(attrs))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coin,
        testing::{
            mock_dependencies_with_balances, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        },
        Addr, OwnedDeps,
    };
    use ibcx_interface::airdrop::{AirdropId, AirdropType, InstantiateMsg};

    use crate::{
        contract::instantiate,
        error::ContractError,
        execute::{
            close,
            fund::fund_event,
            tests::{
                mock_bearer_airdrop, mock_open_airdrop, normalize_amount, register_airdrop,
                Balances,
            },
        },
    };

    use super::fund;

    fn setup(airdrop_type: AirdropType) -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let env = mock_env();

        let initial_balances: Balances = &[];

        let mut deps = mock_dependencies_with_balances(initial_balances);

        instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            InstantiateMsg {},
        )
        .unwrap();

        match airdrop_type {
            AirdropType::Open => {
                let mock_airdrop = mock_open_airdrop(None, env.block.height);
                register_airdrop(deps.as_mut(), env, mock_airdrop.into(), None);
            }
            AirdropType::Bearer => {
                let (mock_airdrop, mock_airdrop_sign) = mock_bearer_airdrop(None, env.block.height);
                register_airdrop(
                    deps.as_mut(),
                    env,
                    mock_airdrop.into(),
                    Some(mock_airdrop_sign),
                );
            }
        }

        deps
    }

    #[test]
    fn test_fund_open() {
        let mut deps = setup(AirdropType::Open);

        // check ok
        {
            let info_fund_creator = mock_info("tester", &[coin(normalize_amount(1.0), "ukrw")]);
            let fund_resp = fund(deps.as_mut(), info_fund_creator, AirdropId::id(0)).unwrap();
            assert_eq!(
                fund_resp.attributes,
                fund_event(
                    Addr::unchecked("tester"),
                    AirdropType::Open,
                    0,
                    normalize_amount(1.0)
                )
            );
        }

        // BEFORE CLOSE
        // check fund
        {
            let info_fund_no_fund = mock_info("tester", &[]);
            let fund_resp = fund(deps.as_mut(), info_fund_no_fund, AirdropId::id(0)).unwrap_err();
            assert_eq!(
                fund_resp,
                ContractError::PaymentError(cw_utils::PaymentError::NoFunds {})
            );

            let info_fund_diff_fund = mock_info("tester", &[coin(10, "ujpy")]);
            let fund_resp = fund(deps.as_mut(), info_fund_diff_fund, AirdropId::id(0)).unwrap_err();
            assert_eq!(
                fund_resp,
                ContractError::PaymentError(cw_utils::PaymentError::MissingDenom(
                    "ukrw".to_string()
                ))
            );

            let info_fund_one_more_funds =
                mock_info("tester", &[coin(10, "ukrw"), coin(10, "ujpy")]);
            let fund_resp =
                fund(deps.as_mut(), info_fund_one_more_funds, AirdropId::id(0)).unwrap_err();
            assert_eq!(
                fund_resp,
                ContractError::PaymentError(cw_utils::PaymentError::MultipleDenoms {})
            );
        }

        // check unauthorized
        {
            let info_fund_abuser = mock_info("abuser", &[coin(normalize_amount(1.0), "ukrw")]);
            let fund_resp = fund(deps.as_mut(), info_fund_abuser, AirdropId::id(0)).unwrap_err();
            assert_eq!(fund_resp, ContractError::Unauthorized {});
        }

        // AFTER CLOSE
        {
            close(
                deps.as_mut(),
                mock_env(),
                mock_info("tester", &[]),
                AirdropId::id(0),
            )
            .unwrap();

            let info_fund_creator = mock_info("tester", &[coin(normalize_amount(1.0), "ukrw")]);
            let fund_resp = fund(deps.as_mut(), info_fund_creator, AirdropId::id(0)).unwrap_err();
            assert_eq!(fund_resp, ContractError::AirdropClosed {});
        }
    }

    #[test]
    fn test_fund_bearer() {
        let mut deps = setup(AirdropType::Bearer);

        {
            let info_fund_creator = mock_info("tester", &[coin(normalize_amount(1.0), "ukrw")]);
            let fund_open_resp = fund(deps.as_mut(), info_fund_creator, AirdropId::id(0)).unwrap();
            assert_eq!(
                fund_open_resp.attributes,
                fund_event(
                    Addr::unchecked("tester"),
                    AirdropType::Bearer,
                    0,
                    normalize_amount(1.0)
                )
            );
        }

        // BEFORE CLOSE
        // check fund
        {
            let info_fund_no_fund = mock_info("tester", &[]);
            let fund_resp = fund(deps.as_mut(), info_fund_no_fund, AirdropId::id(0)).unwrap_err();
            assert_eq!(
                fund_resp,
                ContractError::PaymentError(cw_utils::PaymentError::NoFunds {})
            );

            let info_fund_diff_fund = mock_info("tester", &[coin(10, "ujpy")]);
            let fund_resp = fund(deps.as_mut(), info_fund_diff_fund, AirdropId::id(0)).unwrap_err();
            assert_eq!(
                fund_resp,
                ContractError::PaymentError(cw_utils::PaymentError::MissingDenom(
                    "ukrw".to_string()
                ))
            );

            let info_fund_one_more_funds =
                mock_info("tester", &[coin(10, "ukrw"), coin(10, "ujpy")]);
            let fund_resp =
                fund(deps.as_mut(), info_fund_one_more_funds, AirdropId::id(0)).unwrap_err();
            assert_eq!(
                fund_resp,
                ContractError::PaymentError(cw_utils::PaymentError::MultipleDenoms {})
            );
        }

        // check unauthorized
        {
            let info_fund_abuser = mock_info("abuser", &[coin(normalize_amount(1.0), "ukrw")]);
            let fund_resp = fund(deps.as_mut(), info_fund_abuser, AirdropId::id(0)).unwrap_err();
            assert_eq!(fund_resp, ContractError::Unauthorized {});
        }

        // AFTER CLOSE
        {
            close(
                deps.as_mut(),
                mock_env(),
                mock_info("tester", &[]),
                AirdropId::id(0),
            )
            .unwrap();

            let info_fund_creator = mock_info("tester", &[coin(normalize_amount(1.0), "ukrw")]);
            let fund_resp = fund(deps.as_mut(), info_fund_creator, AirdropId::id(0)).unwrap_err();
            assert_eq!(fund_resp, ContractError::AirdropClosed {});
        }
    }
}
