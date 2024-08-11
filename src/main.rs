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

//////////////////////////////////////////////////////////////////////////
// WILL COMPILE
//////////////////////////////////////////////////////////////////////////
// fn make_and_print_account() {
//     let account = Account::new(1, String::from("me"));
//     println!("{:#?}", account)
// }

//////////////////////////////////////////////////////////////////////////
// WILL NOT COMPILE
//////////////////////////////////////////////////////////////////////////
fn make_and_print_account() -> &Account{ //error[E0106]: missing lifetime specifier
    //                         ^ expected named lifetime parameter
    let account = Account::new(1, String::from("me"));
    println!("{:#?}", account)
    &account // error[E0369]: no implementation for `() & Account`
}


fn main() {
    //////////////////////////////////////////////////////////////////////////
    // WILL COMPILE
    //////////////////////////////////////////////////////////////////////////
    //make_and_print_account();

    //////////////////////////////////////////////////////////////////////////
    // WILL NOT COMPILE
    //////////////////////////////////////////////////////////////////////////
    let account_ref = make_and_print_account();
    println!("{}", account_ref.balance);

}