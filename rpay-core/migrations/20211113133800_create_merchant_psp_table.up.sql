create table if not exists merchant_psp
(
    psp_id         uuid not null,
    psp_feature_id uuid not null,
    PRIMARY KEY (psp_id, psp_feature_id),
    constraint fk_psp foreign key (psp_id) references psp (id),
    constraint fk_psp_feature foreign key (psp_feature_id) references psp_features (id)
);