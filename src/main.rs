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
    //let bank = Bank::new();  

    // NOTE: value in bank has been moved to other_bank
    // warning: unused variable: `other_bank`
    //let other_bank = bank;

    // error[E0382]: use of moved value: `bank`
    //print_bank(bank);
           //  ^^^^ value used here after move


    let account = Account::new(1, String::from("me")); 

    // error[E0382]: use of moved value: `account`
    //print_account(account);
               // ^^^^^^^ this parameter takes ownership of the value
   
    //print_account(account);
              //  ^^^^^^^ value used here after move

    // error[E0382]: borrow of moved value: `account`
    let list_of_accounts = vec![account];
    println!("{:#?}", account);
                   // ^^^^^^^ value borrowed here after move

}
