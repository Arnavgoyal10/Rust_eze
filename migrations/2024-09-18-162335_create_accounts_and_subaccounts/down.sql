-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS pending_transactions CASCADE;
DROP TABLE IF EXISTS transactions CASCADE;
DROP TABLE IF EXISTS sub_accounts CASCADE;
DROP TABLE IF EXISTS accounts CASCADE;