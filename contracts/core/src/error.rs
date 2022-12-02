use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] cosmwasm_std::OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] cosmwasm_std::DivideByZeroError),

    #[error("{0}")]
    CheckedFromRatioError(#[from] cosmwasm_std::CheckedFromRatioError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] cosmwasm_std::CheckedMultiplyRatioError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    ParseReplyError(#[from] cw_utils::ParseReplyError),

    #[error("denom {reserved:?} already reserved")]
    DenomReserved { reserved: String },

    #[error("length of assets exceeded. limit:{limit:?}")]
    InvalidAssetLength { limit: u32 },

    #[error("Paused")]
    Paused {},

    #[error("Not paused")]
    NotPaused {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unknown reply id {id:?}")]
    UnknownReplyId { id: u64 },

    #[error("Received funds mismatched (denom: {denom:?} => required: {required:?}, received: {received:?})")]
    MismatchedFunds {
        denom: String,
        required: Uint128,
        received: Uint128,
    },
}
