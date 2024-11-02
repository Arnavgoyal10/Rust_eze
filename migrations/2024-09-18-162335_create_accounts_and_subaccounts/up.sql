-- Your SQL goes here
-- Create accounts table
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_holder_name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    status VARCHAR NOT NULL DEFAULT 'active'
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