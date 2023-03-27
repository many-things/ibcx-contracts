use cosmwasm_std::{attr, Decimal, DepsMut, MessageInfo, Response};

use crate::{
    error::ContractError,
    state::{get_units, Rebalance, GOV, LATEST_REBALANCE_ID, REBALANCES},
};

// initialize the rebalance
// deflation: target unit of each denom to decrease
// inflation: weight of each denom to distribute
//
// basic flow of rebalance
//
//=========================================
// [ DEFLATION ]            [ INFLATION ]
//-----------------------------------------
//     | A  --\             /-->  D |
//     | B  ---> [RESERVE] ---->  E |
//     | C  --/             \-->  F |
//=========================================
pub fn init(
    deps: DepsMut,
    info: MessageInfo,
    manager: String,
    deflation: Vec<(String, Decimal)>,
    inflation: Vec<(String, Decimal)>,
) -> Result<Response, ContractError> {
    // only governance can execute this
    if info.sender != GOV.load(deps.storage)? {
        return Err(ContractError::Unauthorized {});
    }

    // check if there is a ongoing rebalance
    let rebalance_id = LATEST_REBALANCE_ID
        .may_load(deps.storage)?
        .unwrap_or_default();
    if let Some(r) = REBALANCES.may_load(deps.storage, rebalance_id)? {
        if !r.finalized {
            return Err(ContractError::RebalanceNotFinalized {});
        }
    }

    // make new rebalance
    let rebalance = Rebalance {
        manager: deps.api.addr_validate(&manager)?,
        deflation,
        inflation,
        finalized: false,
    };

    // fetch current units and validate new rebalance
    let units = get_units(deps.storage)?;
    rebalance.validate(units)?;

    // save
    REBALANCES.save(deps.storage, rebalance_id, &rebalance)?;

    let resp = Response::new().add_attributes(vec![
        attr("method", "rebalance_init"),
        attr("executor", info.sender),
        attr("manager", manager),
        attr("rebalance_id", rebalance_id.to_string()),
    ]);

    Ok(resp)
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{testing::mock_info, Addr};

    use crate::test::{mock_dependencies, register_units, to_units, SENDER_GOV, SENDER_OWNER};

    use super::*;

    #[test]
    fn test_init() {
        let mut deps = mock_dependencies();

        GOV.save(deps.as_mut().storage, &Addr::unchecked(SENDER_GOV))
            .unwrap();

        register_units(deps.as_mut().storage, &[("ukrw", "1.0"), ("uusd", "1.8")]);

        let resp = init(
            deps.as_mut(),
            mock_info(SENDER_GOV, &[]),
            "manager".to_string(),
            to_units(&[("ukrw", "0.3")]),
            to_units(&[("ujpy", "1")]),
        )
        .unwrap();
        assert_eq!(
            resp.attributes,
            vec![
                attr("method", "rebalance_init"),
                attr("executor", SENDER_GOV),
                attr("manager", "manager"),
                attr("rebalance_id", "0"),
            ]
        );

        let rebalance = REBALANCES.load(deps.as_ref().storage, 0).unwrap();
        assert_eq!(
            rebalance,
            Rebalance {
                manager: Addr::unchecked("manager"),
                deflation: to_units(&[("ukrw", "0.3")]),
                inflation: to_units(&[("ujpy", "1")]),
                finalized: false
            }
        );
    }

    #[test]
    fn test_check_authority() {
        let mut deps = mock_dependencies();

        GOV.save(deps.as_mut().storage, &Addr::unchecked(SENDER_GOV))
            .unwrap();

        let err = init(
            deps.as_mut(),
            mock_info(SENDER_OWNER, &[]),
            "manager".to_string(),
            to_units(&[]),
            to_units(&[]),
        )
        .unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});
    }

    #[test]
    fn test_check_previous_rebalance() {
        let mut deps = mock_dependencies();

        GOV.save(deps.as_mut().storage, &Addr::unchecked(SENDER_GOV))
            .unwrap();

        REBALANCES
            .save(
                deps.as_mut().storage,
                0,
                &Rebalance {
                    manager: Addr::unchecked("manager"),
                    deflation: vec![],
                    inflation: vec![],
                    finalized: false,
                },
            )
            .unwrap();

        let err = init(
            deps.as_mut(),
            mock_info(SENDER_GOV, &[]),
            "manager".to_string(),
            to_units(&[]),
            to_units(&[]),
        )
        .unwrap_err();
        assert_eq!(err, ContractError::RebalanceNotFinalized {});
    }
}
