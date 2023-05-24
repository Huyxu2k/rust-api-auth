-- Add migration script here
CREATE TABLE IF NOT EXISTS USERS (
    id integer primary key auto_increment NOT NULL,
    email nvarchar(500) NOT NULL UNIQUE,
    username nvarchar(500) NOT NULL,
    password nvarchar(500) NOT NULL,
    login_session nvarchar(500) NOT NULL default '',
    subscribed_at timestamp NOT NULL
);