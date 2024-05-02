CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255) UNIQUE NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  phone_number VARCHAR(20) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE users
ADD COLUMN user_type INTEGER NOT NULL DEFAULT 0;



CREATE TABLE wallets (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  balance DECIMAL(10, 2) DEFAULT 0.00,
  currency VARCHAR(3) NOT NULL DEFAULT 'GMD',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  sender_id INTEGER NOT NULL REFERENCES users(id),
  recipient_id INTEGER NOT NULL REFERENCES users(id),
  amount DECIMAL(10, 2) NOT NULL,
  currency VARCHAR(3) NOT NULL DEFAULT 'GMD',
  status VARCHAR(20) NOT NULL,
  transaction_type VARCHAR(20) NOT NULL,
  transaction_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP

);

Alter Table transactions
ADD Column transaction_number VARCHAR(255);


CREATE TABLE agents (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  phone_number VARCHAR(20),
  latitude DECIMAL(10, 8) NOT NULL,
  longitude DECIMAL(11, 8) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cash_transactions (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  agent_id INTEGER NOT NULL REFERENCES agents(id),
  amount DECIMAL(10, 2) NOT NULL,
  currency VARCHAR(3) NOT NULL DEFAULT 'GMD',
  transaction_type VARCHAR(20) NOT NULL,
  transaction_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE merchants (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  latitude DECIMAL(10, 8) NOT NULL,
  longitude DECIMAL(11, 8) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE merchants
ADD COLUMN email VARCHAR(255),
ADD COLUMN password VARCHAR(255),
ADD COLUMN business_name VARCHAR(255),
ADD COLUMN business_type VARCHAR(100),
ADD COLUMN address TEXT,
ADD COLUMN business_phone_number VARCHAR(20),
ADD COLUMN website VARCHAR(255)
ADD COLUMN edited_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;

Alter Table merchants
ADD Column user_id INTEGER NOT NULL REFERENCES users(id);

ALTER TABLE merchants
DROP COLUMN name,
DROP COLUMN email,
DROP COLUMN password,





CREATE TABLE merchants_payments (
  id SERIAL PRIMARY KEY,
  merchant_id INTEGER NOT NULL REFERENCES merchants(id),
  user_id INTEGER NOT NULL REFERENCES users(id),
  amount DECIMAL(10, 2) NOT NULL,
  currency VARCHAR(3) NOT NULL DEFAULT 'GMD',
  payment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);



CREATE TABLE telecom_operators (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE airtime_topups (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  operator_id INTEGER NOT NULL REFERENCES telecom_operators(id),
  amount DECIMAL(10, 2) NOT NULL,
  currency VARCHAR(3) NOT NULL DEFAULT 'GMD',
  topup_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE utility_providers (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE bill_payments (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  provider_id INTEGER NOT NULL REFERENCES utility_providers(id),
  amount DECIMAL(10, 2) NOT NULL,
  currency VARCHAR(3) NOT NULL DEFAULT 'GMD',
  billing_start_date DATE NOT NULL,
  billing_end_date DATE NOT NULL,
  payment_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE rewards (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  points INTEGER DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE rewards_redemptions (
  id SERIAL PRIMARY KEY,
  reward_id INTEGER NOT NULL REFERENCES rewards(id),
  redemption_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE users
ADD COLUMN verification_code VARCHAR(50),
ADD COLUMN verified BOOLEAN DEFAULT false,
ADD COLUMN verification_code_created_at TIMESTAMP


ALTER TABLE agents
ADD COLUMN verification_code VARCHAR(50),
ADD COLUMN verified BOOLEAN DEFAULT false,
ADD COLUMN verification_code_created_at TIMESTAMP,
ADD COLUMN updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;






CREATE TABLE products_and_services (
    id SERIAL PRIMARY KEY,
    merchant_id INTEGER NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
	title TEXT,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
	is_product BOOLEAN NOT NULL DEFAULT TRUE,
	is_discounted BOOLEAN NOT NULL DEFAULT FALSE,
    discounted_amount DECIMAL(10, 2),
    on_sale BOOLEAN NOT NULL DEFAULT FALSE,
    on_sale_amount DECIMAL(10, 2),
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Drop the existing constraint on the id column
ALTER TABLE products_and_services DROP CONSTRAINT products_and_services_pkey;

-- Alter the id column to be an INTEGER type
ALTER TABLE products_and_services ALTER COLUMN id SET DATA TYPE INTEGER;

-- Drop the sequence if exists
DROP SEQUENCE IF EXISTS products_and_services_id_seq;

-- Create a new sequence for the id column
CREATE SEQUENCE products_and_services_id_seq START 1;

-- Set the default value for the id column to use the sequence
ALTER TABLE products_and_services ALTER COLUMN id SET DEFAULT nextval('products_and_services_id_seq');

-- Add a new primary key constraint on the id column
ALTER TABLE products_and_services ADD PRIMARY KEY (id);




-- QR Codes table
CREATE TABLE qr_codes (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL REFERENCES products_and_services(id),
    data TEXT NOT NULL
);



ALTER TABLE merchants_payments
ADD COLUMN product_id INTEGER NOT NULL REFERENCES products_and_services(id),
ADD COLUMN status VARCHAR(50) NOT NULL DEFAULT 'pending',
ADD COLUMN edited_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;


CREATE TABLE unverified_users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(255) UNIQUE,
  password_hash VARCHAR(255),
  phone_number VARCHAR(20) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE unverified_users
ADD COLUMN verification_code VARCHAR(50)

ALTER TABLE unverified_users
ADD COLUMN phone_verified BOOLEAN DEFAULT false




-- Inserting sample data into the users table
INSERT INTO users (username, password_hash, phone_number, email)
VALUES
  ('john_doe', '12345', '+1234567890', 'john@example.com'),
  ('jane_smith', '12345', '+1987654321', 'jane@example.com');

-- Inserting sample data into the wallets table
INSERT INTO wallets (user_id, balance, currency)
VALUES
  (1, 1000.00, 'GMD'),
  (2, 500.00, 'GMD');

-- Inserting sample data into the transactions table
INSERT INTO transactions (sender_id, recipient_id, amount, currency, status, transaction_type, transaction_date)
VALUES
  (1, 2, 200.00, 'GMD', 'completed', 'transfer', CURRENT_TIMESTAMP),
  (2, 1, 50.00, 'GMD', 'completed', 'transfer', CURRENT_TIMESTAMP);

-- Inserting sample data into the agents table
INSERT INTO agents (name, phone_number, latitude, longitude)
VALUES
  ('Agent 1', '+1122334455', 37.7749, -122.4194),
  ('Agent 2', '+9988776655', 34.0522, -118.2437);

-- Inserting sample data into the cash_transactions table
INSERT INTO cash_transactions (user_id, agent_id, amount, currency, transaction_type, transaction_date)
VALUES
  (1, 1, 100.00, 'GMD', 'deposit', CURRENT_TIMESTAMP),
  (2, 2, 75.00, 'GMD', 'withdrawal', CURRENT_TIMESTAMP);

-- Inserting sample data into the merchants table
INSERT INTO merchants (name, description, latitude, longitude)
VALUES
  ('Merchant A', 'Description for Merchant A', 40.7128, -74.0060),
  ('Merchant B', 'Description for Merchant B', 34.0522, -118.2437);

-- Inserting sample data into the merchants_payments table
INSERT INTO merchants_payments (merchant_id, user_id, amount, currency, payment_date)
VALUES
  (1, 1, 50.00, 'GMD', CURRENT_TIMESTAMP),
  (2, 2, 30.00, 'GMD', CURRENT_TIMESTAMP);

-- Inserting sample data into the telecom_operators table
INSERT INTO telecom_operators (name)
VALUES
  ('Operator X'),
  ('Operator Y');

-- Inserting sample data into the airtime_topups table
INSERT INTO airtime_topups (user_id, operator_id, amount, currency, topup_date)
VALUES
  (1, 1, 20.00, 'GMD', CURRENT_TIMESTAMP),
  (2, 2, 15.00, 'GMD', CURRENT_TIMESTAMP);

-- Inserting sample data into the utility_providers table
INSERT INTO utility_providers (name)
VALUES
  ('Provider A'),
  ('Provider B');

-- Inserting sample data into the bill_payments table
INSERT INTO bill_payments (user_id, provider_id, amount, currency, billing_start_date, billing_end_date, payment_date)
VALUES
  (1, 1, 50.00, 'GMD', '2024-02-01', '2024-02-28', CURRENT_TIMESTAMP),
  (2, 2, 30.00, 'GMD', '2024-02-01', '2024-02-28', CURRENT_TIMESTAMP);

-- Inserting sample data into the rewards table
INSERT INTO rewards (user_id, points)
VALUES
  (1, 100),
  (2, 75);

-- Inserting sample data into the rewards_redemptions table
INSERT INTO rewards_redemptions (reward_id, redemption_date)
VALUES
  (1, CURRENT_TIMESTAMP),
  (2, CURRENT_TIMESTAMP);
