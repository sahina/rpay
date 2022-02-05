create table if not exists psp_features
(
    id      uuid        not null,
    psp_id  uuid        not null,
    feature VARCHAR(16) NOT NULL,
    PRIMARY KEY (id),
    constraint fk_psp foreign key (psp_id) references psp (id)
);