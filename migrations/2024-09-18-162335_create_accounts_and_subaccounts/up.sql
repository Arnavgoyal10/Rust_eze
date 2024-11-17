-- Your SQL goes here

-- Create accounts table
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_holder_name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    status VARCHAR NOT NULL DEFAULT 'active'
);

--Create USERNAME and PASSWORD table 
CREATE TABLE username_password (
    username VARCHAR PRIMARY KEY,
    passwd VARCHAR NOT NULL,
    totp_secret VARCHAR,
    account_id UUID REFERENCES accounts(id) ON DELETE CASCADE
);

-- Create sub_accounts table
CREATE TABLE sub_accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID REFERENCES accounts(id) ON DELETE CASCADE,
    currency VARCHAR NOT NULL,
    balance DOUBLE PRECISION NOT NULL DEFAULT 0.00,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create transactions table 
CREATE TABLE transactions (
    transaction_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sub_account_id_from UUID REFERENCES sub_accounts(id) ON DELETE CASCADE,
    sub_account_id_to UUID REFERENCES sub_accounts(id) ON DELETE CASCADE,
    amount DOUBLE PRECISION NOT NULL,
    transfer_currency VARCHAR NOT NULL,
    transaction_date TIMESTAMP NOT NULL DEFAULT NOW()
);

--Create pending transactions table
CREATE TABLE pending_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id_to_add UUID REFERENCES accounts(id) ON DELETE CASCADE,
    amount DOUBLE PRECISION NOT NULL,
    transfer_currency VARCHAR NOT NULL,
    transaction_date TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Insert admin account with a known UUID
INSERT INTO accounts (id, account_holder_name, status)
VALUES ('00000000-0000-0000-0000-000000000000', 'SYSTEM_ADMIN', 'active');

-- Create admin sub-accounts with infinite balance for major currencies
INSERT INTO sub_accounts (account_id, currency, balance)
VALUES 
    ('00000000-0000-0000-0000-000000000000', 'USD', 999999999999.99),
    ('00000000-0000-0000-0000-000000000000', 'EUR', 999999999999.99),
    ('00000000-0000-0000-0000-000000000000', 'GBP', 999999999999.99),
    ('00000000-0000-0000-0000-000000000000', 'JPY', 999999999999.99),
    ('00000000-0000-0000-0000-000000000000', 'INR', 999999999999.99),
    ('00000000-0000-0000-0000-000000000000', 'SGD', 999999999999.99),
    ('00000000-0000-0000-0000-000000000000', 'AUD', 999999999999.99),
    ('00000000-0000-0000-0000-000000000000', 'EUR', 999999999999.99);

-- Insert admin username and password
INSERT INTO username_password (username, passwd, account_id)
VALUES ('admin', '6969', '00000000-0000-0000-0000-000000000000');