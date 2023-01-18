use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    #[error("Cooldown not expired")]
    CooldownNotExpired {},

    #[error("Rebalance not finalized")]
    RebalanceNotFinalized {},

    #[error("Rebalance finalized")]
    RebalanceFinalized {},

    #[error("Unable to finalize")]
    UnableToFinalize {},

    #[error("Insufficient balance")]
    InsufficientBalance {},

    #[error("Over slippage allowance")]
    OverSlippageAllowance {},
}
