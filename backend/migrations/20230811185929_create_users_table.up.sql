-- Add up migration script here
CREATE TABLE IF NOT EXISTS users
(
    id  serial ,
    email VARCHAR(255) UNIQUE NOT NULL PRIMARY KEY,
    password VARCHAR(255) NOT NULL
)
