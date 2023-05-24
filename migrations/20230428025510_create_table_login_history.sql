-- Add migration script here
CREATE TABLE IF NOT EXISTS login_history 
(
    id integer primary key auto_increment NOT NULL,
    user_id integer NOT NULL REFERENCES users(id),
    login_timestamp TIMESTAMP NOT NULL
);