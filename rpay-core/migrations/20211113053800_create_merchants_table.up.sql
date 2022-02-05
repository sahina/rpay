CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table if not exists merchants
(
    id   uuid DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);