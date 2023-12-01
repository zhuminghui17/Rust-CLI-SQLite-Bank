extern crate rusqlite;
use rusqlite::{params, Connection, Result};
use std::io::{self, Write}; 

enum Command {
    CreateAccount,
    Deposit,
    Withdraw,
    Transfer,
    ViewTransactions,
    Exit,
    Invalid,
}

// Database operation functions go here, such as:
// fn create_account(conn: &Connection, account_holder: &str, initial_deposit: f64) -> Result<()>
fn create_account(conn: &Connection, account_holder: &str, initial_deposit: f64) -> Result<()> {
    conn.execute(
        "INSERT INTO accounts (account_holder, balance) VALUES (?1, ?2)",
        params![account_holder, initial_deposit],
    )?;

    let account_id = conn.last_insert_rowid();
    println!("Account created successfully with ID: {}", account_id);

    Ok(())
}
// fn deposit(conn: &Connection, account_id: i32, amount: f64) -> Result<()>
fn deposit(conn: &Connection, account_id: i32, amount: f64) -> Result<()> {
    // First, check if the account exists and get the current balance
    let mut stmt = conn.prepare("SELECT balance FROM accounts WHERE account_id = ?1")?;
    let balance: f64 = stmt.query_row(params![account_id], |row| row.get(0))?;

    // Calculate the new balance
    let new_balance = balance + amount;

    // Update the account with the new balance
    conn.execute(
        "UPDATE accounts SET balance = ?2 WHERE account_id = ?1",
        params![account_id, new_balance],
    )?;

    // Optionally, insert a record into the transactions table
    conn.execute(
        "INSERT INTO transactions (account_id, amount, transaction_type, timestamp)
         VALUES (?1, ?2, 'deposit', CURRENT_TIMESTAMP)",
        params![account_id, amount],
    )?;

    println!("Deposit of ${} successful for account ID {}", amount, account_id);
    Ok(())
}

// fn withdraw(conn: &Connection, account_id: i32, amount: f64) -> Result<()>
fn withdraw(conn: &Connection, account_id: i32, amount: f64) -> Result<()> {
    let mut stmt = conn.prepare("SELECT balance FROM accounts WHERE account_id = ?1")?;
    let balance: f64 = stmt.query_row(params![account_id], |row| row.get(0))?;

    if balance < amount {
        // Use a different error handling approach
        return Err(rusqlite::Error::ExecuteReturnedResults);
    }

    let new_balance = balance - amount;
    conn.execute(
        "UPDATE accounts SET balance = ?2 WHERE account_id = ?1",
        params![account_id, new_balance],
    )?;

    conn.execute(
        "INSERT INTO transactions (account_id, amount, transaction_type, timestamp)
         VALUES (?1, ?2, 'withdrawal', CURRENT_TIMESTAMP)",
        params![account_id, amount],
    )?;

    println!("Withdrawal of ${} successful for account ID {}", amount, account_id);
    Ok(())
}

// fn transfer(conn: &Connection, from_account_id: i32, to_account_id: i32, amount: f64) -> Result<()>
fn transfer(conn: &Connection, from_account_id: i32, to_account_id: i32, amount: f64) -> Result<()> {
    withdraw(conn, from_account_id, amount)?;
    deposit(conn, to_account_id, amount)?;

    println!("Transfer of ${} from account ID {} to account ID {} successful", amount, from_account_id, to_account_id);
    Ok(())
}

// fn view_transactions(conn: &Connection, account_id: i32) -> Result<()>
fn view_transactions(conn: &Connection, account_id: i32) -> Result<()> {
    let mut stmt = conn.prepare("SELECT transaction_id, amount, transaction_type, timestamp FROM transactions WHERE account_id = ?1 ORDER BY timestamp DESC")?;
    let transaction_iter = stmt.query_map(params![account_id], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;

    for transaction in transaction_iter {
        let (transaction_id, amount, transaction_type, timestamp) = transaction?;
        println!("Transaction ID: {}, Amount: ${}, Type: {}, Timestamp: {}", transaction_id, amount, transaction_type, timestamp);
    }

    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("rust_sqlite_bank.db")?;

    // Create the tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            account_id INTEGER PRIMARY KEY,
            account_holder TEXT NOT NULL,
            balance REAL NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            transaction_id INTEGER PRIMARY KEY,
            account_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            transaction_type TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY(account_id) REFERENCES accounts(account_id)
        )",
        [],
    )?;

    loop {
        println!("Banking Menu:");
        println!("1. Create Account");
        println!("2. Deposit Money");
        println!("3. Withdraw Money");
        println!("4. Transfer Money");
        println!("5. View Transaction History");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let cmd = match choice.trim().parse::<u32>() {
            Ok(1) => Command::CreateAccount,
            Ok(2) => Command::Deposit,
            Ok(3) => Command::Withdraw,
            Ok(4) => Command::Transfer,
            Ok(5) => Command::ViewTransactions,
            Ok(6) => Command::Exit,
            _ => Command::Invalid,
        };

        match cmd {
            Command::CreateAccount => {
                println!("Enter account holder name: ");
                let mut account_holder = String::new();
                io::stdin().read_line(&mut account_holder).expect("Failed to read name");

                println!("Enter initial deposit amount: ");
                let mut deposit_str = String::new();
                io::stdin().read_line(&mut deposit_str).expect("Failed to read amount");
                let initial_deposit: f64 = deposit_str.trim().parse().expect("Enter a valid number");

                create_account(&conn, &account_holder.trim(), initial_deposit)?;
            },

            Command::Deposit => {
                print!("Enter account ID: ");
                io::stdout().flush().unwrap();
                let mut account_id_str = String::new();
                io::stdin().read_line(&mut account_id_str).expect("Failed to read account ID");
                let account_id: i32 = account_id_str.trim().parse().expect("Please enter a valid number");

                print!("Enter deposit amount: ");
                io::stdout().flush().unwrap();
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).expect("Failed to read amount");
                let amount: f64 = amount_str.trim().parse().expect("Please enter a valid number");

                deposit(&conn, account_id, amount)?;
            },

            Command::Withdraw => {
                print!("Enter account ID: ");
                io::stdout().flush().unwrap();
                let mut account_id_str = String::new();
                io::stdin().read_line(&mut account_id_str).expect("Failed to read account ID");
                let account_id: i32 = account_id_str.trim().parse().expect("Please enter a valid number");

                print!("Enter withdrawal amount: ");
                io::stdout().flush().unwrap();
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).expect("Failed to read amount");
                let amount: f64 = amount_str.trim().parse().expect("Please enter a valid number");

                withdraw(&conn, account_id, amount)?;
            },

            Command::Transfer => {
                print!("Enter from account ID: ");
                io::stdout().flush().unwrap();
                let mut from_account_id_str = String::new();
                io::stdin().read_line(&mut from_account_id_str).expect("Failed to read account ID");
                let from_account_id: i32 = from_account_id_str.trim().parse().expect("Please enter a valid number");

                print!("Enter to account ID: ");
                io::stdout().flush().unwrap();
                let mut to_account_id_str = String::new();
                io::stdin().read_line(&mut to_account_id_str).expect("Failed to read account ID");
                let to_account_id: i32 = to_account_id_str.trim().parse().expect("Please enter a valid number");

                print!("Enter transfer amount: ");
                io::stdout().flush().unwrap();
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).expect("Failed to read amount");
                let amount: f64 = amount_str.trim().parse().expect("Please enter a valid number");

                transfer(&conn, from_account_id, to_account_id, amount)?;
            },

            Command::ViewTransactions => {
                print!("Enter account ID to view transactions: ");
                io::stdout().flush().unwrap();
                let mut account_id_str = String::new();
                io::stdin().read_line(&mut account_id_str).expect("Failed to read account ID");
                let account_id: i32 = account_id_str.trim().parse().expect("Please enter a valid number");

                view_transactions(&conn, account_id)?;
            },

            Command::Exit => break,
            Command::Invalid => println!("Invalid option, please try again."),
        }
    }

    Ok(())     
}


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    // Initializes an in-memory database for testing
    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE accounts (
                account_id INTEGER PRIMARY KEY,
                account_holder TEXT NOT NULL,
                balance REAL NOT NULL
            )",
            [],
        ).unwrap();

        conn.execute(
            "CREATE TABLE transactions (
                transaction_id INTEGER PRIMARY KEY,
                account_id INTEGER NOT NULL,
                amount REAL NOT NULL,
                transaction_type TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                FOREIGN KEY(account_id) REFERENCES accounts(account_id)
            )",
            [],
        ).unwrap();

        conn
    }

    #[test]
    fn test_create_account() {
        let conn = setup_db();
        create_account(&conn, "Alice", 1000.0).unwrap();

        let mut stmt = conn.prepare("SELECT account_holder, balance FROM accounts WHERE account_holder = ?1").unwrap();
        let result = stmt.query_row(params!["Alice"], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        });

        match result {
            Ok((name, balance)) => {
                assert_eq!(name, "Alice");
                assert_eq!(balance, 1000.0);
            },
            _ => panic!("Account not found or incorrect data"),
        }
    }

    #[test]
    fn test_deposit() {
        let conn = setup_db();
        create_account(&conn, "Bob", 500.0).unwrap();
        let account_id = conn.last_insert_rowid();
        deposit(&conn, account_id as i32, 200.0).unwrap();

        let balance: f64 = conn.query_row(
            "SELECT balance FROM accounts WHERE account_id = ?1",
            params![account_id],
            |row| row.get(0),
        ).unwrap();

        assert_eq!(balance, 700.0);
    }

    #[test]
    fn test_withdraw() {
        let conn = setup_db();
        create_account(&conn, "Charlie", 1000.0).unwrap();
        let account_id = conn.last_insert_rowid();
        withdraw(&conn, account_id as i32, 300.0).unwrap();

        let balance: f64 = conn.query_row(
            "SELECT balance FROM accounts WHERE account_id = ?1",
            params![account_id],
            |row| row.get(0),
        ).unwrap();

        assert_eq!(balance, 700.0);
    }

    #[test]
    fn test_transfer() {
        let conn = setup_db();
        create_account(&conn, "Dave", 1000.0).unwrap();
        let from_account_id = conn.last_insert_rowid();

        create_account(&conn, "Eve", 500.0).unwrap();
        let to_account_id = conn.last_insert_rowid();

        transfer(&conn, from_account_id as i32, to_account_id as i32, 200.0).unwrap();

        let from_balance: f64 = conn.query_row(
            "SELECT balance FROM accounts WHERE account_id = ?1",
            params![from_account_id],
            |row| row.get(0),
        ).unwrap();

        let to_balance: f64 = conn.query_row(
            "SELECT balance FROM accounts WHERE account_id = ?1",
            params![to_account_id],
            |row| row.get(0),
        ).unwrap();

        assert_eq!(from_balance, 800.0);
        assert_eq!(to_balance, 700.0);
    }

    #[test]
    fn test_view_transactions() {
        let conn = setup_db();
        create_account(&conn, "Frank", 1000.0).unwrap();
        let account_id = conn.last_insert_rowid();

        deposit(&conn, account_id as i32, 200.0).unwrap();
        withdraw(&conn, account_id as i32, 150.0).unwrap();

        let mut stmt = conn.prepare("SELECT transaction_type, amount FROM transactions WHERE account_id = ?1").unwrap();
        let mut rows = stmt.query(params![account_id]).unwrap();

        let mut transactions = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            transactions.push((row.get::<_, String>(0).unwrap(), row.get::<_, f64>(1).unwrap()));
        }

        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0], ("deposit".to_string(), 200.0));
        assert_eq!(transactions[1], ("withdrawal".to_string(), 150.0));
    }
}
