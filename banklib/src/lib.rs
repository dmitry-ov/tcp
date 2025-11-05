use std::io::{Read, Write};
use std::net::TcpStream;

use protocol_crate::{Command, Response, BankError, Operation};

const SERVER_ADDRESS: &str = "127.0.0.1:7878";

pub struct BankClient {
    server_address: String,
}

impl BankClient {
    pub fn new(x: &str) -> Self {
        BankClient {
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
        let response = self.send_command(Command::CreateAccount(account));
        match response {
            Response::Account(result) => Ok(result?),
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
        let response = self.send_command(Command::IncreaseAccount(account, amount));
        match response {
            Response::OperationResult(Ok(_)) => Ok(()),
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
        let response = self.send_command(Command::DecreaseAccount(account, amount));
        match response {
            Response::OperationResult(Ok(_)) => Ok(()),
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
        let response = self.send_command(Command::Transfer { from, to, amount });
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
        let response = self.send_command(Command::GetAccountBalance(account));
        match response {
            Response::AccountBalance(Ok(result)) => Ok(result),
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
        let response = self.send_command(Command::GetHistory);
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
        let response = self.send_command(Command::GetAccountHistory(account));
        match response {
            Response::AccountHistory(result) => result.unwrap(),
            _ => panic!("Unexpected account_history response: {:?}", response),
        }
    }

    /// Restores the bank state from the given `operations`.
    ///
    /// # Arguments
    ///
    /// * `operations` - The operations to be restored.
    pub fn restore(&self, operations: Vec<Operation>) {
        let response = self.send_command(Command::Restore(operations));
        match response {
            Response::Restore => (),
            _ => panic!("Unexpected account_history response: {:?}", response),
        }
    }

    /// Sends a command to the server and waits for the response.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to be sent.
    ///
    /// # Returns
    ///
    /// * `Response` - The response from the server.
    fn send_command(&self, command: Command) -> Response {
        let mut stream = TcpStream::connect(&self.server_address).unwrap();
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
        response
    }
}
