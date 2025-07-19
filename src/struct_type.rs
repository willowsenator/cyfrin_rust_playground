#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub balance: u64,
}

pub fn new(address: String) -> Account {
    Account {
        address,
        balance: 0,
    }
}
