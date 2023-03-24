use cosmwasm_std::{Addr, StdError};
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
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] cosmwasm_std::CheckedMultiplyRatioError),

    #[error("{0}")]
    ParseReplyError(#[from] cw_utils::ParseReplyError),

    #[error("{0}")]
    FromHexError(#[from] hex::FromHexError),

    #[error("{0}")]
    VerificationError(#[from] cosmwasm_std::VerificationError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Key already exists. {key:?}")]
    KeyAlreadyExists { typ: String, key: String },

    #[error("Already claimed. id:{airdrop_id:?}, claim_key:{claim_key:?}")]
    AlreadyClaimed { airdrop_id: u64, claim_key: String },

    #[error("Wrong length")]
    WrongLength {},

    #[error("Invalid proof")]
    InvalidProof {},

    #[error("Airdrop has closed")]
    AirdropClosed {},

    #[error("Invalid arguments. arg:{arg:?}, reason:{reason:?}")]
    InvalidArguments { arg: String, reason: String },

    #[error("Unabled to claim more than supplied funds.")]
    InsufficientAirdropFunds {},

    #[error("Invalid airdrop type. expected:{expected:?}, actual:{actual:?}")]
    InvalidAirdropType { expected: String, actual: String },

    #[error("Invalid signature. action:{action:?}")]
    InvalidSignature { action: String },

    #[error("Invalid public key")]
    InvalidPubKey {},
}

impl ContractError {
    pub fn invalid_airdrop_type(expected: impl ToString, actual: impl ToString) -> Self {
        ContractError::InvalidAirdropType {
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }

    pub fn invalid_signature(action: impl ToString) -> Self {
        ContractError::InvalidSignature {
            action: action.to_string(),
        }
    }
}
