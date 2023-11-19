DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    role VARCHAR(36) NOT NULL DEFAULT 'user',
    email VARCHAR(100) NOT NULL,
    password VARCHAR(122) NOT NULL,
    created_by VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(36) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

insert into users (id, first_name, last_name, role, email, password, created_by, updated_by) values
    ('00000000-0000-0000-0000-000000000000', 'name', 'last name', 'admin', 'admin@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'),
    ('1802d2f8-1a18-43c1-9c58-1c3f7100c842', 'name', 'last name', 'user', 'test@admin.com', '123', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000');
