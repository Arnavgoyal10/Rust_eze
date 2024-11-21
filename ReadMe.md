 Rust EZE Banking System

A robust command-line banking system built in Rust that provides secure account management, multi-currency transactions, and scheduled payment capabilities.

## Table of Contents
- [Features](#features)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Security](#security)
- [API Documentation](#api-documentation)
- [Contributing](#contributing)
- [License](#license)

## Features

### Account Management
- User account creation with secure authentication
- Sub-account creation supporting multiple currencies
- Real-time balance tracking
- Detailed transaction history

### Transaction Capabilities
- Internal transfers between sub-accounts with automatic currency conversion
- External transfers to other user accounts
- Direct deposits to sub-accounts
- Scheduled/recurring transactions
- Pending transaction approval system

### Multi-Currency Support
- Supports 7 major currencies: USD, EUR, GBP, JPY, INR, SGD, AUD
- Real-time currency conversion using ExchangeRate API
- Automatic currency validation
- Amount validation and precision handling

### Security Features
- Password hashing using bcrypt
- Two-factor authentication using TOTP
- Admin interface with separate authentication
- Input validation and sanitization
- Transaction verification system

## Architecture

### Technology Stack
- **Backend**: Rust with Actix-web framework
- **Database**: PostgreSQL with Diesel ORM
- **Authentication**: bcrypt + TOTP
- **External Services**: ExchangeRate API for currency conversion
- **Notifications**: Telegram Bot API

### Core Components
1. **Account Manager**: Handles account creation and management
2. **Transaction Engine**: Processes all financial transactions
3. **Scheduler**: Manages recurring payments and scheduled transactions
4. **Security Layer**: Handles authentication and authorization
5. **Currency Service**: Manages currency conversions and validations

## Prerequisites

- Rust (latest stable version)
- PostgreSQL 12+
- Docker
- Python 3.7+ (for TOTP generation)
- Telegram Bot Token (for notifications)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/Arnavgoyal10/Rust_eze.git
cd Rust_eze
```
2. Install dependencies:
```bash
cargo build
```

3. Run a docker container for postgres:
```bash
docker run --name Rust_eze -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres
```

4. Set up environment variables in .env file
```
DATABASE_URL=postgres://admin:password@localhost:5432/Rust_eze
TELEGRAM_BOT_TOKEN=your_telegram_bot_token
TELEGRAM_CHAT_ID=your_telegram_chat_id
```

5. Run database migrations to initialize the database:
```bash
diesel migration run
```

6. Set up a cronjob to run the recurring transactions every day:
```bash
crontab -e
0 0 * * * cd /path/to/rust_eze && /path/to/.cargo/bin/cargo run --bin recurring_payments >> /path/to/rust_eze/cron.log 2>&1
```
Note: Replace /path/to/rust_eze with the actual path to the Rust_eze directory.

## Usage

### Basic Operations

1. Start the application:
```bash
cargo run --bin main
```

2. Create a new account:
```
1. Select "Create Account"
2. Enter account holder name
3. Set up username and password
4. Save TOTP secret for 2FA
```

3. Login to your account:
```
1. Select "Login"
2. Enter username and password
3. Enter TOTP code for 2FA
```

4. Create a sub-account:
```
1. Select "Create Sub-account"
2. Choose currency
3. Enter initial deposit amount
4. Confirm sub-account creation
```

5. Add funds to your sub-account:
```
1. Select "Add Funds"
2. Choose sub-account
3. Enter amount to add
4. Wait for admin approval of transaction
```

6. Check your balance:
```
1. Select "Check Balance"
2. Choose sub-account
3. View current balance
```

7. Perform transactions:
```
2. Choose transaction type
3. Enter transaction details
4. Confirm transaction
```

8. Check your transaction history:
```
1. Select "Check Transaction History"
2. Choose sub-account
3. View transaction history
```

9. Add a recurring transaction:
```
1. Select "Add Recurring Transaction"
2. Enter transaction details
3. Set recurrence schedule
4. Confirm transaction
```

10. Delete a scheduled transaction:
```
1. Select "View Scheduled Transactions"
2. Copy transaction ID of the transaction to delete
3. Select "Delete Scheduled Transaction"
4. Enter transaction ID to delete
5. Confirm deletion
```

### Admin Operations

1. Login to your account as admin:
```
1. Select "Admin Login"
2. Enter admin username and password (admin/admin)
```

2. Approve pending transactions:
```
1. Select "View Pending Transactions"
2. Copy the transaction ID of the transaction to approve
3. Select "Approve Transaction"
4. Enter transaction ID to approve
5. Confirm approval
```

3. View scheduled transactions:
```
1. Select "View Scheduled Transactions"
2. View scheduled transactions
```


## Security

### Authentication Flow
1. Password verification using bcrypt
2. TOTP verification using PyOTP
3. Session management with secure tokens

### Transaction Security
- All amounts validated before processing
- Currency codes verified against whitelist
- Transaction limits enforced
- Audit trail maintained


## Future Work
- Add post-quantum cryptography for enhanced security
- Integrate with Blockchain for secure transactions
- Asynchronous processing for improved performance
- Improved UI/UX for better user experience
