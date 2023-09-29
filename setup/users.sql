CREATE TYPE ACCESS_LEVEL AS ENUM ('regular', 'admin');

CREATE TABLE IF NOT EXISTS UserLogin (
    id SERIAL PRIMARY KEY,
    password_hash BIGINT,
    salt BIGINT,
    email VARCHAR,
    access_level ACCESS_LEVEL,
);

CREATE TABLE IF NOT EXISTS UserIdentity (
    id INT,
    username VARCHAR(32),
    discriminator SMALLINT,
    account_creation TIMESTAMP WITH TIME ZONE,
    avatar INT,

    PRIMARY KEY (username, discriminator),
    FOREIGN KEY (avatar) REFERENCES UserAvatars(id),
    FOREIGN KEY (id) REFERENCES UserLogin(id),
);

CREATE TABLE IF NOT EXISTS UserAvatars (
    id SERIAL PRIMARY KEY,
    file_name VARCHAR,
    file_data BYTEA NOT NULL,
);