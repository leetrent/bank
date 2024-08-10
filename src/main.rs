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

fn change_account(account: &mut Account) {
    println!("Old balance: {:#?}", account.balance);
    account.balance = 10;
    println!("New balance: {:#?}", account.balance);
}



fn main() {
    /////////////////////////////////////////////////////////////////////////
    // HAPPY PATH
    /////////////////////////////////////////////////////////////////////////
    // let mut account = Account::new(1, String::from("me"));
    // change_account(&mut account);
    // println!("{:#?}", account);

    /////////////////////////////////////////////////////////////////////////
    // Will NOT COMPILE
    /////////////////////////////////////////////////////////////////////////
    // let mut account = Account::new(1, String::from("me"));
    // let account_ref = &account;

    // change_account(&mut account); // error[E0502]: cannot borrow `account` as mutable because it is also borrowed as immutable
    // //             ^^^^^^^^^^^^ mutable borrow occurs here

    // println!("{:#?}", account_ref.holder);

    ///////////////////////////////////////////////////////////////////////////
    // Will NOT COMPILE
    // No read-only references can be in use while there is a mutable reference
    ///////////////////////////////////////////////////////////////////////////
    // let mut account = Account::new(1, String::from("me"));
    // let account_ref = &account;

    //change_account(&mut account); // error[E0502]: cannot borrow `account` as mutable because it is also borrowed as immutable
    //             ^^^^^^^^^^^^ mutable borrow occurs here

    //println!("{:#?}", account_ref.holder);

    ///////////////////////////////////////////////////////////////////////////
    // Will NOT COMPILE
    // Only onw mutable reference at a time is allowed
    ///////////////////////////////////////////////////////////////////////////
    // let mut account = Account::new(1, String::from("me"));
    // let account_ref = &mut account;

    // change_account(&mut account); // error[E0499]: cannot borrow `account` as mutable more than once at a time
    // //             ^^^^^^^^^^^^ second mutable borrow occurs here


    //println!("{:#?}", account_ref.holder);

    ///////////////////////////////////////////////////////////////////////////
    // Will COMPILE
    ///////////////////////////////////////////////////////////////////////////
    // let mut account = Account::new(1, String::from("me"));
    // println!("{:#?}", account);
    // account.balance = 100;
    // println!("{:#?}", account);

    ///////////////////////////////////////////////////////////////////////////
    // Will NOT COMPILE
    // Mutating the value through the owner is not allowed when any references
    // (mutable of immutable) to the value exist.
    ///////////////////////////////////////////////////////////////////////////
    let mut account = Account::new(1, String::from("me"));
    let account_ref = &mut account;

    account.balance = 100;  // error[E0506]: cannot assign to `account.balance` because it is borrowed
//  ^^^^^^^^^^^^^^^^^^^^^ `account.balance` is assigned to here but it was already borrowed
    change_account(account_ref);
    println!("{:#?}", account);

}
