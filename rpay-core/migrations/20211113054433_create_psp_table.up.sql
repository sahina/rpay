create table if not exists psp
(
    id       uuid DEFAULT uuid_generate_v4(),
    name     VARCHAR(255) NOT NULL,
    features VARCHAR(16)[],
    PRIMARY KEY (id)
);