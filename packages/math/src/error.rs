use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("base must be greater than 0")]
    NegativeBase,

    #[error("base must be less than two")]
    BaseTooLarge,

    #[error("{0}")]
    DecimalError(#[from] cosmwasm_std::Decimal256RangeExceeded),
}
