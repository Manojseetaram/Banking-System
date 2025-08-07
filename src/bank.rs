// src/bank.rs

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum BankError {
    AccountExists,
    AccountNotFound,
    InsufficientBalance,
}

#[derive(Debug)]
pub struct Bank {
    accounts: HashMap<String, f64>,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn create_account(&mut self, name: String) -> Result<(), BankError> {
        if self.accounts.contains_key(&name) {
            return Err(BankError::AccountExists);
        }
        self.accounts.insert(name, 0.0);
        Ok(())
    }

    pub fn deposit(&mut self, name: &str, amount: f64) -> Result<(), BankError> {
        let balance = self
            .accounts
            .get_mut(name)
            .ok_or(BankError::AccountNotFound)?;
        *balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, name: &str, amount: f64) -> Result<(), BankError> {
        let balance = self
            .accounts
            .get_mut(name)
            .ok_or(BankError::AccountNotFound)?;
        if *balance < amount {
            return Err(BankError::InsufficientBalance);
        }
        *balance -= amount;
        Ok(())
    }

    pub fn get_balance(&self, name: &str) -> Result<f64, BankError> {
        let balance = self.accounts.get(name).ok_or(BankError::AccountNotFound)?;
        Ok(*balance)
    }
}
