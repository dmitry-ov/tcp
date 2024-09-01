
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Command {
    CreateAccount(String),
    IncreaseAccount(String, u32),
    DecreaseAccount(String, u32),
    Transfer {
        from: String,
        to: String,
        amount: u32,
    },
    GetHistory,
    GetAccountBalance(String),
    Restore(Vec<Operation>),
    GetAccountHistory(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation {
    CreateAccount(String),
    IncreaseAccount(String, u32),
    DecreaseAccount(String, u32),
    Transfer(String, String, u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Account(Result<usize, BankError>),
    OperationResult(Result<usize, BankError>),
    TransferResult(Result<(), BankError>),
    History(Vec<Operation>),
    AccountBalance(Result<u32, BankError>),
    AccountHistory(Option<Vec<Operation>>),
    Restore,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BankError {
    AccountAlreadyExists(String),
    IncorrectAmount(u32),
    InsufficientFunds(u32),
    TransferToMyself,
    AccountDoesNotExist(String)
}
