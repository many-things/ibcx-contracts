use cosmwasm_std::Uint128;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

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

    #[error("{0}")]
    CheckedFromRatioError(#[from] cosmwasm_std::CheckedFromRatioError),

    #[error("Paused")]
    Paused {},

    #[error("Not paused")]
    NotPaused {},

    #[error("Invalid reply id")]
    InvalidReplyId {},

    #[error("Invalid trade route")]
    InvalidTradeRoute {},

    #[error("Trade amount exceeded")]
    TradeAmountExceeded {},

    #[error("Swap route not found. from:{from:?}, to:{to:?}")]
    SwapRouteNotFound { from: String, to: String },

    #[error(
        "Simulate query error. err:{err:?}, input:{input:?}, output:{output:?}, amount:{amount:?}"
    )]
    SimulateQueryError {
        err: String,
        input: String,
        output: String,
        amount: Uint128,
    },
}
