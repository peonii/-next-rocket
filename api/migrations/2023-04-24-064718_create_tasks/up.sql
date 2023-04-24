-- Your SQL goes here
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
)