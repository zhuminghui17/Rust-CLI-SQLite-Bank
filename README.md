# Project #2: Rust CLI Binary with SQLite - CURD in Bank Transcation Data

## Introduction
This application is a command-line banking system implemented in Rust, using SQLite for data storage. It allows users to create bank accounts, deposit and withdraw funds, transfer money between accounts, and view transaction histories.

## Requirements
- Rust Programming Language
- SQLite

## Usage
The application provides a text-based menu with the following options:

1. Create Account: Create a new bank account with an initial deposit.
2. Deposit Money: Deposit funds into an existing account.
3. Withdraw Money: Withdraw funds from an existing account.
4. Transfer Money: Transfer funds between two existing accounts.
5. View Transaction History: View the transaction history of an existing account.
6. Exit: Exit the application.

## Usage Example
Below are some examples of how you might use the application to perform various banking operations. These examples use fictitious data for demonstration purposes.

1. Creating a New Account
When you select the option to create an account, you will be prompted to enter the account holder's name and the initial deposit amount.
```
Input: 
1. Create Account
Enter account holder name: Alice Johnson
Enter initial deposit amount: 1000

Output:
Account created successfully with ID: 1

```

2. Depositing Money
To deposit money, choose the deposit option, enter the account ID, and specify the amount to deposit.

```
Input:
2. Deposit Money
Enter account ID: 1
Enter deposit amount: 500

Output:
Deposit of $500 successful for account ID 1
```


3. Withdrawing Money

For withdrawing money, input the account ID and the amount you wish to withdraw.
```
Input:

3. Withdraw Money
Enter account ID: 1
Enter withdrawal amount: 200

Output:
Withdrawal of $200 successful for account ID 1
```

4. Transferring Money
To transfer money, enter the IDs of the from and to accounts and the transfer amount.
```
Input:
4. Transfer Money
Enter from account ID: 1
Enter to account ID: 2
Enter transfer amount: 300

Output:
Transfer of $300 from account ID 1 to account ID 2 successful
```

5. Viewing Transaction History
Select the option to view transactions and enter the account ID for which you want to see the history.
```
Input:
5. View Transaction History
Enter account ID to view transactions: 1

Output:
Transaction ID: 1, Amount: $1000, Type: deposit, Timestamp: 2023-07-15T10:00:00
Transaction ID: 2, Amount: $500, Type: deposit, Timestamp: 2023-07-16T11:30:00
Transaction ID: 3, Amount: $200, Type: withdrawal, Timestamp: 2023-07-17T09:20:00
Transaction ID: 4, Amount: $300, Type: transfer, Timestamp: 2023-07-18T14:45:00
```

## Testing
The application includes a basic testing suite for its core functionalities.

### Explanation of the Test Cases
1. **`test_create_account`**: Tests if creating an account correctly inserts a record into the `accounts` table with the specified name and balance.

2. **`test_deposit`**: First, an account is created. Then, a deposit is made, and the test checks if the balance is updated correctly in the database.

3. **`test_withdraw`**: Similar to the deposit test, but for withdrawals. It verifies if the balance decreases correctly after a withdrawal.

4. **`test_transfer`**: This test involves creating two accounts and performing a transfer between them. It verifies that the funds are correctly deducted from one account and added to the other.

5. **`test_view_transactions`**: Checks if transactions (both deposit and withdrawal) are recorded correctly. It verifies the transaction type and amount for accuracy.

### Running the Tests

Run the tests using the following Cargo command:
```
cargo test
```

Ensure Rust and its package manager, Cargo, are installed on your system. SQLite is used through the `rusqlite`` crate, so you do not need to install SQLite separately.

## Requirements:

Your project should include the following:
- Rust source code: The code should comprehensively understand Rust's syntax and unique features.
- Use of GitHub Copilot: In your README, explain how you utilized GitHub Copilot in your coding process.
- SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
- Optimized Rust Binary: Include a process that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.
- README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how GitHub Copilot was used.
- GitHub Actions: A workflow file that tests, builds, and lints your Rust code.
- Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

## Grading Rubric for Project #2: Rust CLI Binary with SQLite

- Rust Source Code (25 points): Your Rust source code is well-structured and demonstrates a clear understanding of Rust's syntax and unique features.

	-	Proper usage of Rust syntax: 8 points
	-	Effective error handling in Rust: 8 points
	-	Implementation of Rust's unique features: 9 points

- SQLite Database (25 points): Demonstrates CRUD operations on the SQLite database.
	-	Create Operation: 6 points
	-	Read Operation: 6 points
	-	Update Operation: 6 points
	-	Delete Operation: 7 points


- Use of GitHub Copilot (10 points):

    - Explanation of the project: 3 points
    - How to run the program: 3 points
    - Dependencies and how to install them: 4 points

- Optimized Rust Binary (10 points): Process included that generates an optimized Rust binary as a GitHub Actions artifact that can be downloaded.


- README.md (10 points): The README.md file is clear and concise and guides the user on how to run the program.
	-	Explanation of the project: 3 points
	-	How to run the program: 3 points
	-	Dependencies and how to install them: 4 points

- GitHub Actions (10 points): Your GitHub Actions file should test, build, and lint your Rust code correctly.
	-	Correct testing of Rust code: 3 points
	-	Correct building of Rust code: 3 points
	-	Correct linting of Rust code: 4 points

- Demo Video (10 points): A 2-5 minute video explaining the project and demonstrating its functionality is included. The video should be high-quality (both audio and visual), not exceed the given time limit, and be linked in the README via a private or public YouTube link.
	-	Clarity of explanation: 3 points
	-	Quality demonstration of the project: 3 points
	-	Quality of video and audio: 4 points

- Total: 100 points




## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
