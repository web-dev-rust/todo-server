CREATE TABLE auth_user (
    email VARCHAR(100) NOT NULL PRIMARY KEY,
    id UUID NOT NULL,
    password VARCHAR(64) NOT NULL, --bcrypt hash
    expires_at TIMESTAMP NOT NULL
);