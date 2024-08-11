#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

// Inherent implementation block
impl Account {
    // Associated function
    fn new(id: u32, holder: String) -> Self {
        Account {
            id, // no assignment is necessary because field name and parameter name are identical
            holder, // no assignment is necessary because field name and parameter name are identical
            balance: 0,
        }
    }
}

fn main() {
    let id = 1;
    let holder = String::from("me");
 
    let account = Account::new(id, holder);
 
    println!("{:#?} {:#?}", id, holder);
}