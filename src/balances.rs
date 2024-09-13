use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, account: String, amount: u128) {
        self.balances.insert(account, amount);
    }

    pub fn get_balance(&self, account: String) -> u128 {
        *self.balances.get(&account).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(caller.clone());
        let to_balance = self.get_balance(to.clone());

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();
        assert_eq!(balances.get_balance("yan".to_string()), 0);
        balances.set_balance("yan".to_string(), 10);

        assert_eq!(balances.get_balance("yan".to_string()), 10);
        assert_eq!(balances.get_balance("bubu".to_string()), 0);
    }

    #[test]
    fn update_balance() {
        let mut balances = Pallet::new();
        balances.set_balance("yan".to_string(), 10);
        balances.set_balance("yan".to_string(), 20);

        assert_eq!(balances.get_balance("yan".to_string()), 20);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::new();
        balances.set_balance("Yan".to_string(), 20);

        assert_eq!(
            balances.transfer("Yan".to_string(), "Bruna".to_string(), 10),
            Ok(())
        );
        assert_eq!(balances.get_balance("Yan".to_string()), 10);
        assert_eq!(balances.get_balance("Bruna".to_string()), 10);
    }
}
