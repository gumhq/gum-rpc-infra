use std::io::Error;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlerkleSerializationError {
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Error, Debug)]
pub enum ProgramParserError {
    #[error("Instruction Data Parsing Error")]
    InstructionParsingError,
    #[error("IO Error {0}")]
    IOError(String),
    #[error("Could not deserialize data")]
    DeserializationError,
    #[error("Data length is invalid.")]
    InvalidDataLength,
    #[error("Unknown anchor account discriminator.")]
    UnknownAccountDiscriminator,
    #[error("Account type is not valid")]
    InvalidAccountType,
    #[error("Uninitialized account type")]
    UninitializedAccount,
    #[error("Account Type Not implemented")]
    AccountTypeNotImplemented,
    #[error("Could not deserialize data: {0}")]
    CustomDeserializationError(String),
}

impl From<std::io::Error> for ProgramParserError {
    fn from(err: Error) -> Self {
        ProgramParserError::IOError(err.to_string())
    }
}

impl From<PlerkleSerializationError> for ProgramParserError {
    fn from(err: PlerkleSerializationError) -> Self {
        ProgramParserError::IOError(err.to_string())
    }
}
