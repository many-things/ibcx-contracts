use cosmwasm_std::StdError;
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
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] cosmwasm_std::CheckedMultiplyRatioError),

    #[error("{0}")]
    ParseReplyError(#[from] cw_utils::ParseReplyError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unknown reply id {0}")]
    UnknownReplyId(u64),

    #[error("Unmanaged token")]
    UnmanagedToken {},

    #[error("Token not found: {0}")]
    TokenNotFound(String),

    #[error("Token already exists: {0}")]
    TokenAlreadyExists(String),
}
