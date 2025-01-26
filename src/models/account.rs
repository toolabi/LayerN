


#[derive(Debug)]
struct Account {
    address: String,
    balance: u128
    
} 

impl Account {
    fn new(address: String, balance: u128) -> Account {
        Account {
            address,
            balance
        }
    }
    fn get_address(&self) -> &String {
        &self.address
    }
    fn get_balance(&self) -> u128 {
        self.balance
    }
    fn set_balance(&mut self, balance: u128) {
        self.balance = balance;
    }
    
}