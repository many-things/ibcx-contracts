use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] cosmwasm_std::OverflowError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("Paused")]
    Paused {},

    #[error("Not paused")]
    NotPaused {},

    #[error("Received funds mismatched (denom: {denom:?} => required: {required:?}, received: {received:?})")]
    MismatchedFunds {
        denom: String,
        required: Uint128,
        received: Uint128,
    },
}
