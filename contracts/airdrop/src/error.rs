use cosmwasm_std::{Addr, StdError};
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

    #[error("{0}")]
    FromHexError(#[from] hex::FromHexError),

    #[error("Key already exists. {key:?}")]
    KeyAlreadyExists { typ: String, key: String },

    #[error("Already claimed. id:{airdrop_id:?}, claimer:{claimer:?}")]
    AlreadyClaimed { airdrop_id: u64, claimer: Addr },

    #[error("WrongLength")]
    WrongLength {},

    #[error("InvalidProof")]
    InvalidProof {},
}
