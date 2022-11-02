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
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] cosmwasm_std::CheckedMultiplyRatioError),

    #[error("Paused")]
    Paused {},

    #[error("Not paused")]
    NotPaused {},

    #[error("Trade amount exceeded")]
    TradeAmountExceeded {},

    #[error("Trade strategy not set")]
    TradeStrategyNotSet {},

    #[error("Trade simulation failed")]
    TradeSimulationFailed {},

    #[error("Trade no allocation")]
    TradeNoAllocation {},

    #[error("Trade cooldown not finished")]
    TradeCooldownNotFinished {},

    #[error("Rebalance info not found or not initialized")]
    RebalanceInfoNotFound {},

    #[error("Rebalance already finished")]
    RebalanceAlreadyFinished {},

    #[error("Rebalance already on going")]
    RebalanceAlreadyOnGoing {},

    #[error("Rebalance condition fulfilled")]
    RebalanceConditionFulfilled {},

    #[error("Rebalance ran out of allocation")]
    RebalanceRanOutOfAllocation {},

    #[error("Received funds mismatched (denom: {denom:?} => required: {required:?}, received: {received:?})")]
    MismatchedFunds {
        denom: String,
        required: Uint128,
        received: Uint128,
    },
}
