use std::io::{Read, Write};
use std::net::TcpStream;

use serde::{Deserialize, Serialize};

// const SERVER_ADDRESS: &str = "127.0.0.1:7878";

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Command {
    CreateAccount(String),
    IncreaseAccount(String, u32),
    DecreaseAccount(String, u32),
    Transfer(String, String, u32),
    GetHistory(),
    GetAccountBalance(String),
    Restore(Vec<Operation>),
    GetAccountHistory(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Account(Result<usize, BankError>),
    OperationResult(Result<usize, BankError>),
    TransferResult(Result<(), BankError>),
    History(Vec<Operation>),
    AccountBalance(Result<u32, BankError>),
    AccountHistory(Vec<Operation>),
    Restore(),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BankError {
    AccountAlreadyExists(String),
    IncorrectAmount(u32),
    InsufficientFunds(u32),
    TransferToMyself,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation {
    CreateAccount(String),
    IncreaseAccount(String, u32),
    DecreaseAccount(String, u32),
    Transfer(String, String, u32),
}

pub struct Lib {
    server_address: String,
}

impl Lib {
    pub fn new(x: &str) -> Self {
        Lib {
            server_address: x.to_string(),
        }
    }

    /// Creates a new account with the given `account` name.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account to be created.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The ID of the newly created account.
    /// * `Err(BankError)` - If the account already exists or there was an error during the process.
    ///
    pub fn create_account(&self, account: String) -> Result<usize, BankError> {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::CreateAccount(account);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> =
            serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail create_account response: {:?}", serde_result);
        };
        match response {
            Response::Account(Ok(result)) => Ok(result),
            Response::Account(Err(_)) => Err(BankError::AccountAlreadyExists(
                "Account Alice already exists".to_string(),
            )),
            _ => panic!("Unexpected create_account response: {:?}", response),
        }
    }

    /// Increases the balance of the given `account` by the given `amount`.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account to be increased.
    /// * `amount` - The amount to be increased.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The ID of the newly created account.
    /// * `Err(BankError)` - If the account already exists or there was an error during the process.
    pub fn increase_account(&self, account: String, amount: u32) -> Result<(), BankError> {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::IncreaseAccount(account, amount);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> =
            serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail increase_account response: {:?}", serde_result);
        };
        match response {
            Response::OperationResult(Ok(_)) => Ok(()),
            Response::OperationResult(Err(error)) => Err(error),
            _ => panic!("Unexpected increase_account response: {:?}", response),
        }
    }

    /// Decreases the balance of the given `account` by the given `amount`.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account to be decreased.
    /// * `amount` - The amount to be decreased.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The ID of the newly created account.
    /// * `Err(BankError)` - If the account already exists or there was an error during the process.
    pub fn decrease_account(&self, account: String, amount: u32) -> Result<(), BankError> {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::DecreaseAccount(account, amount);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> =
            serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail decrease_account response: {:?}", serde_result);
        };
        match response {
            Response::OperationResult(Ok(_)) => Ok(()),
            Response::OperationResult(Err(error)) => Err(error),
            _ => panic!("Unexpected decrease_account response: {:?}", response),
        }
    }

    /// Transfers money from one account to another.
    ///
    /// # Arguments
    ///
    /// * `from` - The name of the account to transfer from.
    /// * `to` - The name of the account to transfer to.
    /// * `amount` - The amount to be transferred.
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The ID of the newly created account.
    /// * `Err(BankError)` - If the account already exists or there was an error during the process.
    pub fn transfer(&self, from: String, to: String, amount: u32) -> Result<(), BankError> {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::Transfer(from, to, amount);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> =
            serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail transfer response: {:?}", serde_result);
        };
        match response {
            Response::TransferResult(Ok(_)) => Ok(()),
            Response::TransferResult(Err(error)) => Err(error),
            _ => panic!("Unexpected transfer response: {:?}", response),
        }
    }

    /// Returns the account history of the given `account`.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account to be returned.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Operation>)` - The account history of the given `account`.
    /// * `Err(BankError)` - If the account does not exist or there was an error during the process.
    pub fn get_account_balance(&self, account: String) -> Result<u32, BankError> {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::GetAccountBalance(account);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> =
            serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail get_account_balance response: {:?}", serde_result);
        };
        match response {
            Response::AccountBalance(Ok(result)) => Ok(result),
            Response::AccountBalance(Err(error)) => Err(error),
            _ => panic!("Unexpected get_account_balance response: {:?}", response),
        }
    }

    /// Returns the account history of the given `account`.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account to be returned.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Operation>)` - The account history of the given `account`.
    /// * `Err(BankError)` - If the account does not exist or there was an error during the process.
    pub fn get_history(&self) -> Vec<Operation> {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::GetHistory();
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> =
            serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail get_history response: {:?}", serde_result);
        };
        match response {
            Response::History(result) => result,
            _ => panic!("Unexpected get_history response: {:?}", response),
        }
    }

    /// Returns the account history of the given `account`.
    ///
    /// # Arguments
    ///
    /// * `account` - The name of the account to be returned.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Operation>)` - The account history of the given `account`.
    /// * `Err(BankError)` - If the account does not exist or there was an error during the process.
    pub fn account_history(&self, account: String) -> Vec<Operation> {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::GetAccountHistory(account);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> =
            serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail account_history response: {:?}", serde_result);
        };
        match response {
            Response::AccountHistory(result) => result,
            _ => panic!("Unexpected account_history response: {:?}", response),
        }
    }

    /// Restores the bank state from the given `operations`.
    ///
    /// # Arguments
    ///
    /// * `operations` - The operations to be restored.
    pub fn restore(&self, operations: Vec<Operation>) {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
        let command = Command::Restore(operations);
        let serialized = serde_json::to_string(&command).unwrap();
        println!("restore serialized: {:?}", serialized);
        stream.write_all(serialized.as_bytes()).unwrap();
    }
}
