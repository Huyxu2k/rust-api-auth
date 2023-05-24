-- Add migration script here
CREATE TABLE IF NOT EXISTS persons
(
    id integer primary key auto_increment NOT NULL,
    fullname nvarchar(100) NOT NULL,
    gender boolean NOT NULL,
    age integer NOT NULL,
    address nvarchar(500) NOT NULL,
    phone VARCHAR(10) NOT NULL,    
    email VARCHAR(500) NOT NULL,
    createby nvarchar(100),
    create_datetime TIMESTAMP,
    updateby nvarchar(100),
    update_datetime TIMESTAMP
)