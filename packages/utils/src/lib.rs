use cosmwasm_std::{
    to_vec, Binary, ContractResult, CustomQuery, QuerierWrapper, QueryRequest, StdError, StdResult,
    Storage, SystemResult,
};
use semver::Version;

pub fn store_version<T, U>(storage: &mut dyn Storage, contract: T, version: U) -> StdResult<()>
where
    T: Into<String> + Clone,
    U: Into<String> + Clone,
{
    let stored = cw2::get_contract_version(storage)?;

    if stored.contract != contract.clone().into() {
        return Err(StdError::generic_err("contract name mismatch"));
    }

    let err_conv = |e: semver::Error| StdError::generic_err(e.to_string());
    let cv = Version::parse(&stored.version).map_err(err_conv)?;
    let nv = Version::parse(&version.clone().into()).map_err(err_conv)?;

    if cv >= nv {
        return Err(StdError::generic_err(
            "contract version must be greater than current",
        ));
    }

    cw2::set_contract_version(storage, contract, version)?;

    Ok(())
}

pub fn raw_query_bin<C: CustomQuery>(
    querier: &QuerierWrapper,
    req: &QueryRequest<C>,
) -> StdResult<Binary> {
    let raw = to_vec(req).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
    })?;
    match querier.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {}",
            system_err
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {}",
            contract_err
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => Ok(value),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::MockStorage;

    #[test]
    fn test_store_version() {
        let mut storage = MockStorage::default();

        cw2::set_contract_version(&mut storage, "test_contract", "0.1.0").unwrap();
        store_version(&mut storage, "test_contract", "0.1.1").unwrap();

        assert_eq!(
            store_version(&mut storage, "test_contract", "0.0.9").unwrap_err(),
            StdError::generic_err("contract version must be greater than current")
        );
        assert_eq!(
            store_version(&mut storage, "another_test_contract", "0.1.1").unwrap_err(),
            StdError::generic_err("contract name mismatch")
        );
    }
}
