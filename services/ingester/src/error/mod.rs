use crate::TaskData;
use gpl_parser::error::ProgramParserError;
use plerkle_messenger::MessengerError;
use sea_orm::DbErr;
use sea_orm::TransactionError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum IngesterError {
    #[error("ChangeLog Event Malformed")]
    ChangeLogEventMalformed,
    #[error("Compressed Asset Event Malformed")]
    CompressedAssetEventMalformed,
    #[error("Error downloading batch files")]
    BatchInitNetworkingError,
    #[error("Error writing batch files")]
    BatchInitIOError,
    #[error("Storage listener error: ({msg})")]
    StorageListenerError { msg: String },
    #[error("Storage Write Error {0}")]
    StorageWriteError(String),
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Deserialization Error {0}")]
    DeserializationError(String),
    #[error("Task Manager Error {0}")]
    TaskManagerError(String),
    #[error("Missing or invalid configuration: ({msg})")]
    ConfigurationError { msg: String },
    #[error("Error getting RPC data {0}")]
    RpcGetDataError(String),
    #[error("RPC returned data in unsupported format {0}")]
    RpcDataUnsupportedFormat(String),
    #[error("Data serializaton error {0}")]
    SerializatonError(String),
    #[error("Messenger error {0}")]
    MessengerError(String),
    #[error("Blockbuster Parsing error {0}")]
    ParsingError(String),
    #[error("Data Base Error {0}")]
    DatabaseError(String),
    #[error("Unknown Task Type {0}")]
    UnknownTaskType(String),
    #[error("BG Task Manager Not Started")]
    TaskManagerNotStarted,
    #[error("Unrecoverable task error")]
    UnrecoverableTaskError,
    #[error("Cache Storage Write Error {0}")]
    CacheStorageWriteError(String),
}

impl From<reqwest::Error> for IngesterError {
    fn from(_err: reqwest::Error) -> Self {
        IngesterError::BatchInitNetworkingError
    }
}

impl From<serde_json::Error> for IngesterError {
    fn from(_err: serde_json::Error) -> Self {
        IngesterError::SerializatonError("JSON ERROR".to_string())
    }
}

impl From<ProgramParserError> for IngesterError {
    fn from(err: ProgramParserError) -> Self {
        IngesterError::ParsingError(err.to_string())
    }
}

impl From<std::io::Error> for IngesterError {
    fn from(_err: std::io::Error) -> Self {
        IngesterError::BatchInitIOError
    }
}

impl From<MessengerError> for IngesterError {
    fn from(e: MessengerError) -> Self {
        IngesterError::MessengerError(e.to_string())
    }
}

impl From<DbErr> for IngesterError {
    fn from(e: DbErr) -> Self {
        IngesterError::StorageWriteError(e.to_string())
    }
}

impl From<TransactionError<IngesterError>> for IngesterError {
    fn from(e: TransactionError<IngesterError>) -> Self {
        IngesterError::StorageWriteError(e.to_string())
    }
}

impl From<SendError<TaskData>> for IngesterError {
    fn from(err: SendError<TaskData>) -> Self {
        IngesterError::TaskManagerError(format!("Could not create task: {:?}", err.to_string()))
    }
}
