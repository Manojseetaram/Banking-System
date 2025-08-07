#[cfg(test)]
mod tests{
    use super::super::bank::*;
    #[test]
    fn test_create_and_get_balance(){
        let mut bank = Bank::new();
        //check condition is true 
        assert!(bank.create_account("Alice" , into()).is_ok());
        //check two values are equal
        assert_eq!(
            bank.get_balance("Alice").unwrap(), 0.0;
        )

    }
    #[test]

    fn test_double_account_cretion(){
        let mut bank = Bank::new();
        bank.create_account("Alice".into()).unwrap();
        assert_eq!(bank.create_account("Alice".into()),Err(BankError::AccountExists));
    }
   #[test]
   fn test_depositeand_withdraw(){
    let mut bank = Bank::new();
    bank.create_account ("manoj".into()).unwrap();
    bank.deposit("manoj" , 500.0).unwrap();
    bank.withdraw("manoj" , 200.0).unwrap();
    assert_eq!(bank.get_balance("manoj"),unwrap(), 300.0);
   }
   #[test]
    fn test_bank_insuffient_funds(){
        let bank = Bank::new();
        bank.create_account("manoj".into()).unwrap();
        bank.deposit("manoj" , 100.0).unwrap();
        let result = bank.withdraw("manoj" , 150.0);
        assert_eq!(result , Err(BankError::InsufficientBalance));
    }
}