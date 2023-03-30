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
pub enum ValidationError {
    #[error("invalid config. field:{field}, reason:{reason}")]
    InvalidConfig { field: String, reason: String },

    #[error("invalid pause info. reason:{reason}")]
    InvalidPauseInfo { reason: String },

    #[error("invalid fee. field:{field}, reason:{reason}")]
    InvalidFee { field: String, reason: String },

    #[error("invalid units. reason:{reason}")]
    InvalidUnits { reason: String },

    #[error("invalid rebalance. field:{field}, reason:{reason}")]
    InvalidRebalance { field: String, reason: String },
}

impl ValidationError {
    pub fn invalid_config(field: impl Into<String>, reason: impl Into<String>) -> Self {
        ValidationError::InvalidConfig {
            field: field.into(),
            reason: reason.into(),
        }
    }

    pub fn invalid_pause_info(reason: impl Into<String>) -> Self {
        ValidationError::InvalidPauseInfo {
            reason: reason.into(),
        }
    }

    pub fn invalid_fee(field: impl Into<String>, reason: impl Into<String>) -> Self {
        ValidationError::InvalidFee {
            field: field.into(),
            reason: reason.into(),
        }
    }

    pub fn invalid_rebalance(field: impl Into<String>, reason: impl Into<String>) -> Self {
        ValidationError::InvalidRebalance {
            field: field.into(),
            reason: reason.into(),
        }
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

    #[error("{0}")]
    ValidationError(#[from] ValidationError),

    #[error("Paused")]
    Paused,

    #[error("Not paused")]
    NotPaused,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Unknown reply id {id:?}")]
    UnknownReplyId { id: u64 },

    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
}
