CREATE TABLE friends (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    user_id_sender BIGINT UNSIGNED NOT NULL,
    user_id_receiver BIGINT UNSIGNED NOT NULL,
    status TINYINT NOT NULL,
    -- Timestamps
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

ALTER TABLE
    friends
ADD
    CONSTRAINT friends_user_id_sender_foreign FOREIGN KEY (user_id_sender) REFERENCES users(id) ON DELETE CASCADE;

ALTER TABLE
    friends
ADD
    CONSTRAINT friends_user_id_receiver_foreign FOREIGN KEY(user_id_receiver) REFERENCES users(id) ON DELETE CASCADE;