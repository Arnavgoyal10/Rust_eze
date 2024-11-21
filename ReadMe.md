# Rust EZE Banking System

A command-line banking system built in Rust that allows users to manage accounts, sub-accounts, and transactions.

## Features

- Account Management
  - Create new accounts with username/password authentication
  - Create sub-accounts with different currencies
  - View account balances
  - Get transaction history

- Transaction Capabilities  
  - Transfer between sub-accounts
  - Transfer money to other users
  - Add money to sub-accounts
  - Schedule future transactions

- Security
  - Password hashing using bcrypt
  - Username/password authentication
  - OTP verification
  - Admin interface with separate credentials

- Multi-Currency Support
  - Supports USD, EUR, GBP, JPY, INR, SGD, AUD
  - Currency validation
  - Amount validation

## Getting Started

1. Clone the repository
    ```bash
    git clone https://github.com/Arnavgoyal10/Rust_eze.git
    ```
2. Install Dependencies
   - Ensure you have Rust and Cargo installed
   - Install PostgreSQL database
   - Install Docker
   - Run `cargo build` to install required packages

2. Database Setup
   - Run a Docker container with PostgreSQL
   - Set database URL in environment variables file .env
   - Run migrations to set up schema by running 'diesel migration run' in the terminal

3. Run the venv
   - Run 'source venv/bin/activate' in the terminal

4. Running the Application
   ```bash
   cargo run
   ```

## Usage

### Creating an Account
1. Select "Create Account" option
2. Enter account name
3. Set up username and password

### Making Transfers
1. Log in with username/password
2. Choose transfer type:
   - Between sub-accounts
   - To another user
3. Enter transfer details (amount, currency, recipient)

### Scheduled Transactions
1. Log in to your account
2. Select "Add scheduled transaction"
3. Enter transaction details and future date

### Admin Interface
1. Log in with admin credentials (username: admin, password: 6969)
2. Select "Admin interface"
3. Perform admin tasks such as approving pending transactions

## Technical Details

Built using:
- Rust with Actix-web framework
- Diesel ORM for database operations
- PostgreSQL database
- bcrypt for password hashing
- UUID for unique identifiers
- Chrono for datetime handling

## Security Notes

- Passwords are hashed before storage
- OTP is generated using the TOTP algorithm
- Input validation for all user inputs
- Date validation for scheduled transactions
- Currency code validation
- Amount validation

## License

This project is open source and available under the MIT License.
