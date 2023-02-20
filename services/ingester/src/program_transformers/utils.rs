use crate::program_transformers::instruction::InstructionBundle;

use plerkle_serialization::AccountInfo;
use solana_sdk::pubkey::Pubkey;

pub enum ProgramParseResult {
    // Bubblegum(&'a BubblegumInstruction),
    Unknown,
}

use std::io::Error;
use thiserror::Error;

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

pub trait ParseResult: Sync + Send {
    fn result_type(&self) -> ProgramParseResult;

    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
}

pub struct NotUsed(());

impl NotUsed {
    pub fn new() -> Self {
        NotUsed(())
    }
}

impl Default for NotUsed {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseResult for NotUsed {
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::Unknown
    }
}

pub trait ProgramParser: Sync + Send {
    fn key(&self) -> Pubkey;
    fn key_match(&self, key: &Pubkey) -> bool;
    fn handles_instructions(&self) -> bool;
    fn handles_account_updates(&self) -> bool;
    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<dyn ParseResult>, ProgramParserError>;
    fn handle_instruction(
        &self,
        bundle: &InstructionBundle,
    ) -> Result<Box<dyn ParseResult>, ProgramParserError>;
}
