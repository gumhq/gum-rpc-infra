use crate::{
    error::ProgramParserError, instruction::InstructionBundle, programs::ProgramParseResult,
};
use plerkle_serialization::{AccountInfo, TransactionInfo};
use solana_sdk::pubkey::Pubkey;

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
