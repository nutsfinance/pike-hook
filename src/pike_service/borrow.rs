use ethers::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct AccountPosition {
    pub borrow_positions: HashMap<U256, (U256, U256)>,
    pub is_collateral: bool,
}

impl AccountPosition {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Default)]
pub struct Account {
    pub address: Address,
    pub debt: Option<U256>,
    pub positions: HashMap<Address, AccountPosition>,
    pub seizable_collateral: Option<Address>,
}

impl Account {
    pub fn new(address: Address, market_map: &HashMap<Address, Abigen>) -> Self {
        let mut markets = HashMap::<Address, AccountPosition>::new();
        for address in market_map.keys() {
            markets.insert(*address, AccountPosition::new());
        }

        Self {
            address,
            positions: markets,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn debt(&self) -> U256 {
        if let Some(debt) = self.debt {
            debt
        } else {
            U256::zero()
        }
    }

    pub fn address(&self) -> H160 {
        self.address
    }
}
