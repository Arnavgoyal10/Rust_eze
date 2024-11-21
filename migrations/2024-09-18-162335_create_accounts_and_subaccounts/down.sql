-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS username_password CASCADE;
DROP TABLE IF EXISTS pending_transactions CASCADE;
DROP TABLE IF EXISTS scheduled_transactions CASCADE;
DROP TABLE IF EXISTS transactions CASCADE;
DROP TABLE IF EXISTS sub_accounts CASCADE;
DROP TABLE IF EXISTS accounts CASCADE;