CREATE TABLE users (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    username VARCHAR(50) UNIQUE NOT NULL,
    motto VARCHAR(255) UNIQUE NOT NULL DEFAULT '',
    look VARCHAR(255) UNIQUE NOT NULL DEFAULT '',

    gender VARCHAR(1) NOT NULL DEFAULT 'M' CHECK (gender IN ('M', 'F')),

    rank INT NOT NULL DEFAULT 1,
    credits INT NOT NULL DEFAULT 0,

    home_room BIGINT DEFAULT NULL,

    auth_ticket VARCHAR(255) UNIQUE DEFAULT NULL,

    account_created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_online TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    current_ip VARCHAR(45) NOT NULL DEFAULT '127.0.0.1',

    machine_id VARCHAR(255) NOT NULL DEFAULT ''
);

CREATE INDEX idx_users_auth_ticket ON users(auth_ticket);

CREATE TABLE users_currency (
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    currency_type INT NOT NULL,
    amount BIGINT NOT NULL DEFAULT 0,

    PRIMARY KEY (user_id, currency_type)
);
