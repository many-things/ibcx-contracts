use cosmwasm_std::{StdError, StdResult, Storage};
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
