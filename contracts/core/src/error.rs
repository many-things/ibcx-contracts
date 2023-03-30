use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RebalanceError {
    #[error("on rebalancing")]
    OnRebalancing,

    #[error("not on rebalancing")]
    NotOnRebalancing,

    #[error("on trade cooldown")]
    OnTradeCooldown,

    #[error("trade error. method:{method:?}, reason:{reason:?}")]
    TradeError { method: String, reason: String },

    #[error("unable to finalize. reason: {0}")]
    UnableToFinalize(String),
}

impl RebalanceError {
    pub fn trade_error(method: impl Into<String>, reason: impl Into<String>) -> Self {
        RebalanceError::TradeError {
            method: method.into(),
            reason: reason.into(),
        }
    }

    pub fn unable_to_finalize(reason: impl Into<String>) -> Self {
        RebalanceError::UnableToFinalize(reason.into())
    }
}

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

    #[error("{0}")]
    RebalanceError(#[from] RebalanceError),

    #[error("denom {reserved:?} already reserved")]
    DenomReserved { reserved: String },

    #[error("not found")]
    NotFound,

    #[error("duplicated denom in units")]
    DenomDuplicated,

    #[error("length of assets exceeded. limit:{limit:?}")]
    InvalidAssetLength { limit: u32 },

    #[error("Paused")]
    Paused,

    #[error("Not paused")]
    NotPaused,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unknown reply id {id:?}")]
    UnknownReplyId { id: u64 },

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),

    #[error("Unable to finalize")]
    UnableToFinalize,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Over slippage allowance")]
    OverSlippageAllowance,
}
