CREATE TABLE user (
    id BIGINT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    created_at DATETIME NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=COMPRESSED;
CREATE TABLE post (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    user_id BIGINT NOT NULL,
    published BOOLEAN NOT NULL,
    created_at DATETIME NOT NULL,
    FOREIGN KEY fk_post_user_id (user_id)
        REFERENCES user (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=COMPRESSED;
CREATE TABLE foot_stamp (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    latitude double NOT NULL,
    longitude double NOT NULL,
    created_at DATETIME NOT NULL,
    FOREIGN KEY fk_foot_stamp_user_id (user_id)
        REFERENCES user (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 ROW_FORMAT=COMPRESSED;