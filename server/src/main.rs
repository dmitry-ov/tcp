use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use std::process;

use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::bank::{Bank, BankError};

mod bank;

#[derive(Parser, Debug)]
#[command(name = "Пример")]
#[command(version = "1.0")]
#[command(about = "Пример использования clap")]
struct Args {
    port: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Command {
    CreateAccount(String),
    IncreaseAccount(String, u32),
    DecreaseAccount(String, u32),
    Transfer(String, String, u32),
    GetHistory(),
    GetAccountBalance(String),
    Restore(Vec<bank::Operation>),
    GetAccountHistory(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Account(Result<usize, BankError>),
    OperationResult(Result<usize, BankError>),
    TransferResult(Result<(), BankError>),
    History(Vec<bank::Operation>),
    AccountBalance(Result<u32, BankError>),
    AccountHistory(Option<Vec<bank::Operation>>),
    Restore,
}

fn handle_request(bank: &mut Bank, mut stream: &TcpStream) -> Response {
    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer).unwrap();

    // Десериализация полученных данных
    let received_data = &buffer[..n];
    let command: Command = serde_json::from_slice(received_data).unwrap();

    // Вывод десериализованных данных
    println!("Received command: {:?}", command);

    // Выполнение команды
    match command {
        Command::CreateAccount(account) => Response::Account(bank.create_account(account)),

        Command::IncreaseAccount(account, amount) => {
            Response::OperationResult(bank.increase_account(account, amount))
        }
        Command::DecreaseAccount(account, amount) => {
            Response::OperationResult(bank.decrease_account(account, amount))
        }
        Command::Transfer(from, to, amount) => {
            Response::TransferResult(bank.transfer(from, to, amount))
        }
        Command::GetHistory() => Response::History(bank.get_history().clone()),
        Command::GetAccountBalance(account) => {
            Response::AccountBalance(bank.get_account_balance(account))
        }
        Command::GetAccountHistory(account) => {
            Response::AccountHistory(bank.get_account_history(account))
        }
        Command::Restore(history) => {
            bank.restore(&history);
            Response::Restore
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.port.is_empty() {
        println!("no params");
        process::exit(1);
    }

    let server_address = "127.0.0.1:".to_string().add(&args.port);
    println!("server_address: {}", &server_address);

    let mut bank: Bank = Bank::default();
    let listener = TcpListener::bind(server_address)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = handle_request(&mut bank, &stream);
                let response_json = serde_json::to_string(&response).unwrap();
                println!("Sent response: {} \n", &response_json);
                let result = stream.write(response_json.as_bytes());
                if let Err(e) = result {
                    eprintln!("Failed to write to stream: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
    Ok(())
}
