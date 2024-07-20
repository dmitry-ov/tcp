use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::bank::BankError::AccountAlreadyExists;

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

#[derive(Debug, Serialize, Deserialize)]
pub enum BankError {
    AccountAlreadyExists(String),
    IncorrectAmount(u32),
    InsufficientFunds(u32),
    TransferToMyself,
}

impl Default for Bank {
    fn default() -> Self {
        Self::new()
    }
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            accounts: HashSet::new(),
            balances: HashMap::new(),
            account_operations_index: HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn get_account_balance(&self, account: String) -> Result<u32, BankError> {
        if !self.accounts.contains(&account) {
            return Err(AccountAlreadyExists(format!(
                "Account {} does not exist",
                account
            )));
        }
        let balance = *self.balances.get(&account).unwrap();
        Ok(balance)
    }

    pub fn create_account(&mut self, account: String) -> Result<usize, BankError> {
        if !self.accounts.contains(&account) {
            self.accounts.insert(account.clone());
            self.balances.insert(account.clone(), 0);
            let id = self.append_history(Operation::CreateAccount(account.clone()));
            self.append_account_index(account, id);
            Ok(id)
        } else {
            Err(AccountAlreadyExists(format!(
                "Account {} already exists",
                account
            )))
        }
    }

    pub fn increase_account(&mut self, account: String, amount: u32) -> Result<usize, BankError> {
        self.check_exists_account(account.clone())?;
        self.check_zero_amount(amount)?;

        let current_balance = *self.balances.get(&account).unwrap();
        let new_balance = current_balance + amount;
        self.balances.insert(account.clone(), new_balance);

        let id = self.append_history(Operation::IncreaseAccount(account.clone(), amount));
        self.append_account_index(account, id);
        Ok(id)
    }

    pub fn decrease_account(&mut self, account: String, amount: u32) -> Result<usize, BankError> {
        self.check_exists_account(account.clone())?;
        self.check_zero_amount(amount)?;

        let current_balance = *self.balances.get(&account).unwrap();
        if current_balance < amount {
            return Err(BankError::InsufficientFunds(amount));
        }

        let new_balance = current_balance - amount;
        self.balances.insert(account.clone(), new_balance);
        let id = self.append_history(Operation::DecreaseAccount(account.clone(), amount));
        self.append_account_index(account, id);
        Ok(id)
    }

    pub fn transfer(&mut self, from: String, to: String, amount: u32) -> Result<(), BankError> {
        if from == to {
            return Err(BankError::TransferToMyself);
        }

        self.check_exists_account(from.clone())?;
        self.check_exists_account(to.clone())?;
        self.check_zero_amount(amount)?;

        let current_balance_from = *self.balances.get(&from.clone()).unwrap();
        if current_balance_from < amount {
            return Err(BankError::InsufficientFunds(amount));
        }
        let new_balance_from = current_balance_from - amount;
        self.balances.insert(from.clone(), new_balance_from);

        let current_balance_to = *self.balances.get(&to.clone()).unwrap();
        let new_balance_to = current_balance_to + amount;
        self.balances.insert(to.clone(), new_balance_to);

        let id = self.append_history(Operation::Transfer(from.clone(), to.clone(), amount));

        self.append_account_index(from, id);
        self.append_account_index(to, id);
        Ok(())
    }

    pub fn get_history(&self) -> &Vec<Operation> {
        &self.history
    }

    pub fn get_account_history(&self, account: String) -> Option<Vec<Operation>> {
        self.account_operations_index
            .get(&account)
            .map(|vec| vec.iter().map(|id| self.history[*id].clone()).collect())
    }

    pub fn restore(&mut self, history: &Vec<Operation>) {
        for operation in history {
            match operation {
                Operation::CreateAccount(account) => {
                    let _ = self.create_account(account.clone());
                }
                Operation::IncreaseAccount(account, amount) => {
                    let _ = self.increase_account(account.clone(), *amount);
                }
                Operation::DecreaseAccount(account, amount) => {
                    let _ = self.decrease_account(account.clone(), *amount).unwrap();
                }
                Operation::Transfer(from, to, amount) => {
                    let _ = self.transfer(from.clone(), to.clone(), *amount);
                }
            }
        }
    }

    fn check_zero_amount(&self, amount: u32) -> Result<(), BankError> {
        if amount == 0 {
            return Err(BankError::IncorrectAmount(amount));
        }
        Ok(())
    }

    fn check_exists_account(&mut self, account: String) -> Result<(), BankError> {
        if !self.accounts.contains(&account) {
            return Err(AccountAlreadyExists(format!(
                "Account {} does not exist",
                account
            )));
        }
        Ok(())
    }

    fn append_history(&mut self, operation: Operation) -> usize {
        self.history.push(operation);
        self.history.len() - 1
    }

    fn append_account_index(&mut self, account: String, id: usize) {
        if self.account_operations_index.contains_key(&account) {
            //для такого аккаунта есть индекс
            self.account_operations_index
                .get_mut(&account)
                .unwrap()
                .push(id);
        } else {
            //для такого аккаунта еще нет индекса
            self.account_operations_index
                .insert(account.clone(), vec![id]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bank() {
        let b = Bank::new();
        assert_eq!(0, b.history.len());
        assert_eq!(0, b.account_operations_index.len());
    }

    #[test]
    fn create_account() {
        let mut b = Bank::new();
        let _ = b.create_account("X".to_string());
        assert_eq!(1, b.history.len());
        assert_eq!(1, b.account_operations_index.len());
    }

    #[test]
    fn create_account_twice() {
        let mut b = Bank::new();
        let _ = b.create_account("X".to_string());
        let x = b.create_account("X".to_string());
        assert!(x.is_err());
    }

    #[test]
    fn increase_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.increase_account("X".to_string(), 10);
        assert!(x.is_ok());
        let balance = bank.balances.get(&"X".to_string()).unwrap();
        assert_eq!(10, *balance);
    }

    #[test]
    fn increase_account_zero() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.increase_account("X".to_string(), 0);
        assert!(x.is_err());
    }

    #[test]
    fn increase_no_account() {
        let mut bank = Bank::new();
        let x = bank.increase_account("X".to_string(), 10);
        assert!(x.is_err());
    }

    #[test]
    fn get_account_balance() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.get_account_balance("X".to_string());
        assert!(x.is_ok());
        assert_eq!(0, x.unwrap());
    }

    #[test]
    fn get_no_account_balance() {
        let bank = Bank::new();
        let x = bank.get_account_balance("X".to_string());
        assert!(x.is_err());
    }

    #[test]
    fn decrease_from_no_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.decrease_account("Y".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn decrease_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.decrease_account("X".to_string(), 5);
        assert!(x.is_ok());
        let balance = bank.balances.get(&"X".to_string()).unwrap();
        assert_eq!(5, *balance);
    }

    #[test]
    fn decrease_no_account() {
        let mut bank = Bank::new();
        let x = bank.decrease_account("X".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn decrease_account_zero() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let x = bank.decrease_account("X".to_string(), 0);
        assert!(x.is_err());
    }

    #[test]
    fn decrease_account_too_much() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.decrease_account("X".to_string(), 20);
        assert!(x.is_err());
    }

    #[test]
    fn transfer() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.create_account("Y".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "Y".to_string(), 5);
        assert!(x.is_ok());
        let balance = bank.balances.get(&"X".to_string()).unwrap();
        assert_eq!(5, *balance);
        let balance = bank.balances.get(&"Y".to_string()).unwrap();
        assert_eq!(5, *balance);
    }

    #[test]
    fn transfer_zero() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.create_account("Y".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "Y".to_string(), 0);
        assert!(x.is_err());
    }

    #[test]
    fn transfer_to_self() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "X".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn transfer_to_no_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let x = bank.transfer("X".to_string(), "Y".to_string(), 5);
        assert!(x.is_err());
    }

    #[test]
    fn history_create_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());

        let id = 0;
        assert_eq!(
            Operation::CreateAccount("X".to_string()),
            *bank.get_history().get(id).unwrap()
        );
        assert_eq!(
            id,
            *bank
                .account_operations_index
                .get("X")
                .unwrap()
                .get(0)
                .unwrap()
        );
    }

    #[test]
    fn history_increase_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);

        let id = 1;
        assert_eq!(
            Operation::IncreaseAccount("X".to_string(), 10),
            *bank.get_history().get(id).unwrap()
        );
        assert_eq!(
            id,
            *bank
                .account_operations_index
                .get("X")
                .unwrap()
                .get(1)
                .unwrap()
        );
    }

    #[test]
    fn history_decrease_account() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let _ = bank.decrease_account("X".to_string(), 5);

        let id = 2;
        assert_eq!(
            Operation::DecreaseAccount("X".to_string(), 5),
            *bank.get_history().get(id).unwrap()
        );
        assert_eq!(
            id,
            *bank
                .account_operations_index
                .get("X")
                .unwrap()
                .get(2)
                .unwrap()
        );
    }

    #[test]
    fn history_transfer() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.create_account("Y".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let _ = bank.transfer("X".to_string(), "Y".to_string(), 5);

        let id = 3;
        assert_eq!(
            Operation::Transfer("X".to_string(), "Y".to_string(), 5),
            *bank.get_history().get(id).unwrap()
        );
        assert_eq!(
            id,
            *bank
                .account_operations_index
                .get("X")
                .unwrap()
                .get(2)
                .unwrap()
        );
        assert_eq!(
            id,
            *bank
                .account_operations_index
                .get("Y")
                .unwrap()
                .get(1)
                .unwrap()
        );
    }

    #[test]
    fn get_account_history() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.create_account("Y".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let _ = bank.decrease_account("X".to_string(), 5);
        let _ = bank.transfer("X".to_string(), "Y".to_string(), 5);
        let history = bank.get_account_history("X".to_string()).unwrap();
        assert_eq!(4, history.len());
        assert_eq!(
            Operation::CreateAccount("X".to_string()),
            *history.get(0).unwrap()
        );
        assert_eq!(
            Operation::IncreaseAccount("X".to_string(), 10),
            *history.get(1).unwrap()
        );
        assert_eq!(
            Operation::DecreaseAccount("X".to_string(), 5),
            *history.get(2).unwrap()
        );
        assert_eq!(
            Operation::Transfer("X".to_string(), "Y".to_string(), 5),
            *history.get(3).unwrap()
        );
    }

    #[test]
    fn get_no_account_history() {
        let bank = Bank::new();
        let history = bank.get_account_history("X".to_string());
        assert!(history.is_none());
    }

    #[test]
    fn restore() {
        let mut bank = Bank::new();
        let _ = bank.create_account("X".to_string());
        let _ = bank.create_account("Y".to_string());
        let _ = bank.increase_account("X".to_string(), 10);
        let _ = bank.transfer("X".to_string(), "Y".to_string(), 5);

        let mut new_bank = Bank::new();
        let _ = new_bank.restore(bank.get_history());
        assert_eq!(4, new_bank.get_history().len());
        assert_eq!(5, new_bank.get_account_balance("X".to_string()).unwrap());
        assert_eq!(5, new_bank.get_account_balance("X".to_string()).unwrap());

        assert_eq!(bank.get_history().len(), new_bank.get_history().len());
    }
}
