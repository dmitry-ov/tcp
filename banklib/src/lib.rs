use std::io::{Read, Write};
use std::net::TcpStream;

use serde::{Deserialize, Serialize};

const SERVER_ADDRESS: &str = "127.0.0.1:7878";

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


pub struct Lib {}

impl Lib {
    pub fn create_account(account: String) -> Result<usize, BankError> {
        let mut stream = TcpStream::connect(crate::SERVER_ADDRESS).unwrap();
        let command = Command::CreateAccount(account);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> = serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail create_account response: {:?}", serde_result);
        };
        match response {
            Response::Account(Ok(result)) => Ok(result),
            Response::Account(Err(_)) => Err(BankError::AccountAlreadyExists("Account Alice already exists".to_string())),
            _ => panic!("Unexpected create_account response: {:?}", response),
        }
    }

    pub fn increase_account(account: String, amount: u32) -> Result<(), BankError> {
        let mut stream = TcpStream::connect(crate::SERVER_ADDRESS).unwrap();
        let command = Command::IncreaseAccount(account, amount);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> = serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail increase_account response: {:?}", serde_result);
        };
        match response {
            Response::OperationResult(Ok(_)) => Ok(()),
            Response::OperationResult(Err(error)) => Err(error),
            _ => panic!("Unexpected increase_account response: {:?}", response),
        }
    }

    pub fn decrease_account(account: String, amount: u32) -> Result<(), BankError> {
        let mut stream = TcpStream::connect(crate::SERVER_ADDRESS).unwrap();
        let command = Command::DecreaseAccount(account, amount);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> = serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail decrease_account response: {:?}", serde_result);
        };
        match response {
            Response::OperationResult(Ok(_)) => Ok(()),
            Response::OperationResult(Err(error)) => Err(error),
            _ => panic!("Unexpected decrease_account response: {:?}", response),
        }
    }

    pub fn transfer(from: String, to: String, amount: u32) -> Result<(), BankError> {
        let mut stream = TcpStream::connect(crate::SERVER_ADDRESS).unwrap();
        let command = Command::Transfer(from, to, amount);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> = serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail transfer response: {:?}", serde_result);
        };
        match response {
            Response::TransferResult(Ok(_)) => Ok(()),
            Response::TransferResult(Err(error)) => Err(error),
            _ => panic!("Unexpected transfer response: {:?}", response),
        }
    }

    pub fn get_account_balance(account: String) -> Result<u32, BankError> {
        let mut stream = TcpStream::connect(crate::SERVER_ADDRESS).unwrap();
        let command = Command::GetAccountBalance(account);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> = serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail get_account_balance response: {:?}", serde_result);
        };
        match response {
            Response::AccountBalance(Ok(result)) => Ok(result),
            Response::AccountBalance(Err(error)) => Err(error),
            _ => panic!("Unexpected get_account_balance response: {:?}", response),
        }
    }

    pub fn get_history() -> Vec<Operation> {
        let mut stream = TcpStream::connect(crate::SERVER_ADDRESS).unwrap();
        let command = Command::GetHistory();
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> = serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail get_history response: {:?}", serde_result);
        };
        match response {
            Response::History(result) => result,
            _ => panic!("Unexpected get_history response: {:?}", response),
        }
    }
    pub fn account_history(account: String) -> Vec<Operation> {
        let mut stream = TcpStream::connect(crate::SERVER_ADDRESS).unwrap();
        let command = Command::GetAccountHistory(account);
        let serialized = serde_json::to_string(&command).unwrap();
        stream.write_all(serialized.as_bytes()).unwrap();

        let mut buffer = [0; 512];
        let n = stream.read(&mut buffer).unwrap();
        let received_data = &buffer[..n];

        let serde_result: Result<Response, serde_json::Error> = serde_json::from_slice(received_data);
        let Ok(response) = serde_result else {
            panic!("Fail account_history response: {:?}", serde_result);
        };
        match response {
            Response::AccountHistory(result) => result,
            _ => panic!("Unexpected account_history response: {:?}", response),
        }
    }
}
