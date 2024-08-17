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

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
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

    // Method
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);

    }

    // Method
    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    // Method
    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();

    //////////////////////////////////////////////////////////////////////////////////
    // Casey's account
    //////////////////////////////////////////////////////////////////////////////////
    let mut account_casey = Account::new(1, String::from("Casey"));  
    account_casey.deposit(1000);
    account_casey.withdraw(250);
    bank.add_account(account_casey);

    //////////////////////////////////////////////////////////////////////////////////
    // Pooh's account
    //////////////////////////////////////////////////////////////////////////////////
    let mut account_pooh = Account::new(1, String::from("Pooh Bear"));  
    account_pooh.deposit(500);
    account_pooh.withdraw(250);
    bank.add_account(account_pooh);

    //////////////////////////////////////////////////////////////////////////////////
    // Bank summary of accounts
    //////////////////////////////////////////////////////////////////////////////////
    println!("{:#?}", bank.summary());
    println!("Total of all account balances: {}", bank.total_balance());
}
