create table if not exists psp_api_keys
(
    id          uuid                  DEFAULT uuid_generate_v4(),
    psp_id      uuid         not null,
    merchant_id uuid         not null,
    api_key     VARCHAR(255) NOT NULL,
    api_secret  VARCHAR(255) NOT NULL,
    api_version VARCHAR(16)  NOT NULL,
    environment VARCHAR(32)  NOT NULL,
    status      VARCHAR(16)  NOT NULL,
    created_at  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id),
    constraint fk_psp foreign key (id) references psp (id),
    constraint fk_merchant foreign key (id) references merchants (id)
);