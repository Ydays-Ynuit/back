CREATE TABLE users (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    -- Key
    public_key VARCHAR(255) NOT NULL,
    -- Auth
    password VARCHAR(255) NOT NULL,
    -- Timestamps
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

ALTER TABLE
    users
ADD
    UNIQUE users_username_unique(username);

ALTER TABLE
    users
ADD
    UNIQUE users_public_key_unique(public_key);