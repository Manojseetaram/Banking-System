// src/tests.rs

#[cfg(test)]
mod tests {
    use super::super::bank::*;

    #[test]
    fn test_create_and_get_balance() {
        let mut bank = Bank::new();
        assert!(bank.create_account("Alice".into()).is_ok());
        assert_eq!(bank.get_balance("Alice").unwrap(), 0.0);
    }

    #[test]
    fn test_double_account_creation() {
        let mut bank = Bank::new();
        bank.create_account("Alice".into()).unwrap();
        assert_eq!(
            bank.create_account("Alice".into()),
            Err(BankError::AccountExists)
        );
    }

    #[test]
    fn test_deposit_and_withdraw() {
        let mut bank = Bank::new();
        bank.create_account("Bob".into()).unwrap();
        bank.deposit("Bob", 500.0).unwrap();
        bank.withdraw("Bob", 200.0).unwrap();
        assert_eq!(bank.get_balance("Bob").unwrap(), 300.0);
    }

    #[test]
    fn test_insufficient_funds() {
        let mut bank = Bank::new();
        bank.create_account("Eve".into()).unwrap();
        bank.deposit("Eve", 100.0).unwrap();
        let result = bank.withdraw("Eve", 150.0);
        assert_eq!(result, Err(BankError::InsufficientBalance));
    }
}
    