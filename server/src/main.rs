mod bank;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde::{Serialize, Deserialize};
use crate::bank::{Bank, BankError};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Command {
    CreateAccount(String),
    IncreaseAccount(String, u32),
    DecreaseAccount(String, u32),
    Transfer(String, String, u32),
    GetHistory(),
    GetAccountBalance(String),
    Restore(Vec<bank::Operation>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Account(Result<usize, BankError>),
    OperationResult(Result<usize, BankError>),
    TransferResult(Result<(), BankError>),
    History(Vec<bank::Operation>),
    AccountBalance(Result<u32, BankError>),
    AccountHistory(Vec<bank::Operation>),
    Restore(),
}


fn handle_request(mut bank: &mut Bank, mut stream: &TcpStream) -> Response {
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer).unwrap();

    // Десериализация полученных данных
    let received_data = &buffer[..n];
    let command: Command = serde_json::from_slice(received_data).unwrap();

    // Вывод десериализованных данных
    println!("Received command: {:?}", command);

    // Выполнение команды
    match command {
        Command::CreateAccount(account) => {
            Response::Account(bank.create_account(account))
        }
        Command::IncreaseAccount(account, amount) => {
            Response::OperationResult(bank.increase_account(account, amount))
        }
        Command::DecreaseAccount(account, amount) => {
            Response::OperationResult(bank.decrease_account(account, amount))
        }
        Command::Transfer(from, to, amount) => {
            Response::TransferResult(bank.transfer(from, to, amount))
        }
        Command::GetHistory() => {
            Response::History(bank.get_history().clone())
        }
        Command::GetAccountBalance(account) => {
            Response::AccountBalance(bank.get_account_balance(account))
        }
        Command::Restore (history) => {
            bank.restore(&history);
            Response::Restore()
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut bank :Bank = Bank::default();
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = handle_request(&mut bank, &stream);
                let response_json = serde_json::to_string(&response).unwrap();
                println!("Send response: {} \n", &response_json);
                stream.write(response_json.as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
    Ok(())
}
