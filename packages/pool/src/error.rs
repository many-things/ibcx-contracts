use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoolError {
    //================ external ================//
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("{0}")]
    Overflow(#[from] cosmwasm_std::OverflowError),

    #[error("{0}")]
    FromRatio(#[from] cosmwasm_std::CheckedFromRatioError),

    #[error("{0}")]
    MultiplyRatio(#[from] cosmwasm_std::CheckedMultiplyRatioError),

    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    //================ ibcx ================//
    #[error("{0}")]
    IBCXMath(#[from] ibcx_math::MathError),

    //================ custom ================//
    #[error("unsupported pool type")]
    UnsupportedPoolType,

    #[error("swap route not found. from:{from:?}, to:{to:?}")]
    SwapRouteNotFound { from: String, to: String },

    #[error("pool not found. pool_id:{0}")]
    PoolNotFound(u64),

    #[error("max loop exceeded")]
    MaxLoopExceeded,

    #[error("trade amount exceeded")]
    TradeAmountExceeded,
}
