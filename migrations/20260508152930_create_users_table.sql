CREATE TABLE users (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    username VARCHAR(50) UNIQUE NOT NULL,
    motto VARCHAR(255) UNIQUE NOT NULL DEFAULT '',
    look VARCHAR(255) UNIQUE NOT NULL DEFAULT '',

    gender VARCHAR(1) NOT NULL DEFAULT 'M' CHECK (gender IN ('M', 'F')),

    rank INT NOT NULL DEFAULT 1,
    credits INT NOT NULL DEFAULT 0,

    home_room INT DEFAULT NULL,

    auth_ticket VARCHAR(255) UNIQUE DEFAULT NULL,

    account_created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_online TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    current_ip VARCHAR(45) NOT NULL DEFAULT '127.0.0.1',

    machine_id VARCHAR(255) NOT NULL DEFAULT ''
);

CREATE INDEX idx_users_auth_ticket ON users(auth_ticket);

CREATE TABLE users_currency (
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    currency_type INT NOT NULL,
    amount INT NOT NULL DEFAULT 0,

    PRIMARY KEY (user_id, currency_type)
);

CREATE TABLE user_stats (
    user_id INT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    respects_received INT NOT NULL DEFAULT 0,
    daily_respects INT NOT NULL,
    daily_pet_respects INT NOT NULL
);

CREATE TABLE user_settings (
    user_id INT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    can_change_name BOOLEAN NOT NULL DEFAULT TRUE,
    safety_locked BOOLEAN NOT NULL DEFAULT FALSE
);
