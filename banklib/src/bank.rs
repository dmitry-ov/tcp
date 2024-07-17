use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

type OperationId = usize;

#[derive(Debug)]
pub struct Bank {
    // Счета
    accounts: HashSet<String>,
    // Балансы
    balances: HashMap<String, u32>,
    // История счета
    account_operations_index: HashMap<String, Vec<OperationId>>,
    // История
    history: Vec<Operation>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation {
    CreateAccount(String),
    IncreaseAccount(String, u32),
    DecreaseAccount(String, u32),
    Transfer(String, String, u32),
}
