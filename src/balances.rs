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
}

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
