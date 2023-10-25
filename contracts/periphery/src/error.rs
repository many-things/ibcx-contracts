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
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] cosmwasm_std::CheckedMultiplyRatioError),

    #[error("{0}")]
    ParseReplyError(#[from] cw_utils::ParseReplyError),

    #[error("{0}")]
    CheckedFromRatioError(#[from] cosmwasm_std::CheckedFromRatioError),

    #[error("{0}")]
    IBCXMath(#[from] ibcx_math::MathError),

    #[error("{0}")]
    IBCXPool(#[from] ibcx_pool::PoolError),

    #[error("Paused")]
    Paused {},

    #[error("Not paused")]
    NotPaused {},

    #[error("Invalid reply id {0}")]
    InvalidReplyId(u64),

    #[error("Invalid trade route")]
    InvalidTradeRoute {},

    #[error("Invalid type of context {0}")]
    InvalidContextType(String),

    #[error("Trade amount exceeded")]
    TradeAmountExceeded {},

    #[error("Swap route not found. from:{from:?}, to:{to:?}")]
    SwapRouteNotFound { from: String, to: String },

    #[error("Pool not found. pool_id:{0}")]
    PoolNotFound(u64),

    #[error(
        "Simulate query error. err:{err:?}, input:{input:?}, output:{output:?}, amount:{amount:?}"
    )]
    SimulateQueryError {
        err: String,
        input: String,
        output: String,
        amount: Uint128,
    },

    #[error("Unsupported pool type")]
    UnsupportedPoolType,

    #[error("Max loop exceeded")]
    MaxLoopExceeded,

    #[error("Invalid index amount range")]
    InvalidIndexAmountRange,
}
