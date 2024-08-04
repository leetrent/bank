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

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

// Inherent implementation block
impl Bank {
    // Associated function
    fn new() -> Self {
        // Implicit return (no 'return' statement and no semicolon at the end of the implicit return statement)
        Bank { accounts: vec![] /*empty vector*/ }
    }
}

fn print_bank(bank: Bank) {
    println!("{:#?}", bank);
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}
fn main() {
    //////////////////////////////////////////////////////////////////
    // Scenario #1:
    /////////////////////////////////////////////////////////////////
    //let bank = Bank::new();  
    //let other_bank = bank;  // ---- value moved here
    //println!("{:#?}", bank); // error[E0382]: borrow of moved value: `bank`
    //                ^^^^ value borrowed here after move

    //////////////////////////////////////////////////////////////////
    // Scenario #2:
    /////////////////////////////////////////////////////////////////
    //let account = Account::new(1, String::from("me")); 
    //print_account(account); // ------- value moved here
    //print_account(account); // error[E0382]: use of moved value: `account`
    //            ^^^^^^^ value used here after move
 
     //////////////////////////////////////////////////////////////////
    // Scenario #3:
    /////////////////////////////////////////////////////////////////
    let account = Account::new(1, String::from("me")); 
    let list_of_accounts = vec![account]; // ------- value moved here
    println!("{:#?}", account); // error[E0382]: borrow of moved value: `account`
    //                ^^^^^^^ value borrowed here after move

}
