-- Your SQL goes here
CREATE TABLE todo(
    id SERIAL PRIMARY KEY, 
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    date timestamp NOT NULL DEFAULT NOW()
)