CREATE TABLE users (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    username VARCHAR(50) UNIQUE NOT NULL,
    motto VARCHAR(255) NOT NULL DEFAULT '',
    look VARCHAR(255) NOT NULL DEFAULT 'hd-180-1.ch-215-62',
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

CREATE INDEX idx_users_auth_ticket ON users (auth_ticket);

CREATE TABLE users_currency (
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    currency_type INT NOT NULL,
    amount INT NOT NULL DEFAULT 0,

    PRIMARY KEY (user_id, currency_type)
);

CREATE TABLE users_stats (
    user_id INT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    respects_received INT NOT NULL DEFAULT 0,
    daily_respects INT NOT NULL,
    daily_pet_respects INT NOT NULL
);

CREATE TABLE users_settings (
    user_id INT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    can_change_name BOOLEAN NOT NULL DEFAULT TRUE,
    safety_locked BOOLEAN NOT NULL DEFAULT FALSE,
    volume_system INT NOT NULL DEFAULT 100,
    volume_furni INT NOT NULL DEFAULT 100,
    volume_trax INT NOT NULL DEFAULT 100,
    old_chat BOOLEAN NOT NULL DEFAULT FALSE,
    room_invites BOOLEAN NOT NULL DEFAULT TRUE,
    camera_follow BOOLEAN NOT NULL DEFAULT TRUE,
    chat_type INT NOT NULL DEFAULT 0
);
