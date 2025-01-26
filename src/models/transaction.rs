use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    hash : String,
    from : String,
    to : String,
    value : u128,
    fee : u128,
    date : String,
}

impl Transaction {
    fn new(hash: String, from: String, to: String, value: u128, fee: u128, date: String) -> Transaction {
        Transaction {
            hash,
            from,
            to,
            value,
            fee,
            date
        }
    }
    fn get_hash(&self) -> &String {
        &self.hash
    }
    fn get_from(&self) -> &String {
        &self.from
    }
    fn get_to(&self) -> &String {
        &self.to
    }
    fn get_value(&self) -> u128 {
        self.value
    }
    fn get_fee(&self) -> u128 {
        self.fee
    }
    fn get_date(&self) -> &String {
        &self.date
    }
    
}