-- Add up migration script here
CREATE TABLE IF NOT EXISTS apod
(
    id         serial PRIMARY KEY,
    title      VARCHAR(255) NOT NULL,
    explanation    TEXT         NOT NULL,
    date        VARCHAR(255)  NOT NULL ,
    hdurl       VARCHAR(255) NOT NULL ,
    copyright   VARCHAR(255),
    url         VARCHAR(255) NOT NULL

);
