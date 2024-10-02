use solana_program::{program_error::ProgramError};
use thiserror::Error;

#[derive(Debug,Error)]
pub enum ReviewError{
    //Error0
    #[error("Account Not Initialized yet")]
    uninitializedAccount,

    //Error 1
    #[error("PDA does not equal to PDA passed in")]
    invalidPDA,

    //Error2
    #[error("input Data exceeds max lenght")]
    InvalidDataLenght,

    //Error3
    #[error("Rating Must be <= 5 or >= 0")]
    InvalidRating,

    //Error4
    #[error("Account Not initialised")]
    AccountNotInitialised,

    #[error("Invalid Owner")]
    InvalidOwner,

    #[error("Invalid Arguments")]
    InvalidArguments
}


impl From<ReviewError> for ProgramError {
    fn from(e: ReviewError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
