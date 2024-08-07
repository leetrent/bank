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

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_holder(holder: String) {
    println!("{}", holder);
}

fn main() {
    //////////////////////////////////////////////////////////////////
    // Scenario #1:
    /////////////////////////////////////////////////////////////////
    let account = Account::new(1, String::from("me"));
    let account_ref = &account;

    print_account(account_ref);
    println!("{:#?}", account);

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
    //let account = Account::new(1, String::from("me")); 
    //let list_of_accounts = vec![account]; // ------- value moved here
    //println!("{:#?}", account); // error[E0382]: borrow of moved value: `account`
    //                ^^^^^^^ value borrowed here after move

    //////////////////////////////////////////////////////////////////
    // Scenario #4:
    /////////////////////////////////////////////////////////////////
    //let bank = Bank::new();
    //let accounts = bank.accounts; // ------------- value moved here
    //println!("{:#?}", bank.accounts); // error[E0382]: borrow of moved value: `bank.accounts`
    //                ^^^^^^^^^^^^^ value borrowed here after move

    //////////////////////////////////////////////////////////////////
    // Scenario #5:
    /////////////////////////////////////////////////////////////////
    //let account = Account::new(1, String::from("me"));
    //print_account(account); // ------- value moved here
    //println!("{}", account.holder); // error[E0382]: borrow of moved value: `account`
    //             ^^^^^^^^^^^^^^ value borrowed here after move

    //////////////////////////////////////////////////////////////////
    // Scenario #6:
    /////////////////////////////////////////////////////////////////
    //let account = Account::new(1, String::from("me"));
    //print_holder(account.holder); // -------------- value partially moved here
    //print_account(account); // error[E0382]: use of partially moved value: `account`
    //            ^^^^^^^ value used here after partial move

}
