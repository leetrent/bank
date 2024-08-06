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

// This function takes ownership of the account value
// and returns the account value
fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}



fn main() {
    // account needs to be mutable so it can be reassigned
    let mut account = Account::new(1, String::from("me"));

    // Gives ownership back to account
    account = print_account(account);

    // Gives ownership back to account
    account = print_account(account);

    // Everything works fine
    // account will be printed for the 3rd time
    println!("{:#?}", account);
}
