use crate::{error::IngesterError, TaskData};
use gpl_parser::instruction::order_instructions;
use gpl_parser::instruction::InstructionBundle;
use gpl_parser::instruction::IxPair;
use gpl_parser::program_handler::ProgramParser;
use plerkle_serialization::{AccountInfo, Pubkey as FBPubkey, TransactionInfo};
use sea_orm::{DatabaseConnection, SqlxPostgresConnector};
use solana_sdk::pubkey::Pubkey;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet, VecDeque};
use tokio::sync::mpsc::UnboundedSender;
// use gpl_parser::{I}

pub struct ProgramTransformer {
    storage: DatabaseConnection,
    task_sender: UnboundedSender<TaskData>,
    matchers: HashMap<Pubkey, Box<dyn ProgramParser>>,
    key_set: HashSet<Pubkey>,
}

impl ProgramTransformer {
    pub fn new(pool: PgPool, task_sender: UnboundedSender<TaskData>) -> Self {
        let mut matchers: HashMap<Pubkey, Box<dyn ProgramParser>> = HashMap::with_capacity(1);
        // let bgum = BubblegumParser {};
        // matchers.insert(bgum.key(), Box::new(bgum));
        let hs = matchers.iter().fold(HashSet::new(), |mut acc, (k, _)| {
            acc.insert(*k);
            acc
        });
        let pool: PgPool = pool;
        ProgramTransformer {
            storage: SqlxPostgresConnector::from_sqlx_postgres_pool(pool),
            task_sender,
            matchers,
            key_set: hs,
        }
    }

    pub fn break_transaction<'i>(
        &self,
        tx: &'i TransactionInfo<'i>,
    ) -> VecDeque<(IxPair<'i>, Option<Vec<IxPair<'i>>>)> {
        let ref_set: HashSet<&[u8]> = self.key_set.iter().map(|k| k.as_ref()).collect();
        order_instructions(ref_set, tx)
    }

    pub fn match_program(&self, key: &FBPubkey) -> Option<&Box<dyn ProgramParser>> {
        self.matchers.get(&Pubkey::new(key.0.as_slice()))
    }

    pub async fn handle_instruction<'a>(
        &self,
        ix: &'a InstructionBundle<'a>,
    ) -> Result<(), IngesterError> {
        if let Some(program) = self.match_program(&ix.program) {
            let result = program.handle_instruction(ix)?;
            let concrete = result.result_type();

            match concrete {
                _ => Err(IngesterError::NotImplemented),
            }?;
        }
        Ok(())
    }

    pub async fn handle_account_update<'b>(
        &self,
        acct: AccountInfo<'b>,
    ) -> Result<(), IngesterError> {
        let owner = acct.owner().unwrap();
        if let Some(program) = self.match_program(owner) {
            let result = program.handle_account(&acct)?;
            let concrete = result.result_type();
            match concrete {
                _ => Err(IngesterError::NotImplemented),
            }?;
        }
        Ok(())
    }
}
