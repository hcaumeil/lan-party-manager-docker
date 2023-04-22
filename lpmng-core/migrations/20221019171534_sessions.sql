create table if not exists sessions
(
    id          uuid    default gen_random_uuid() not null primary key,
    ip4         text                              not null,
    user_id     uuid,
    internet    boolean default false             not null,
    date_time   timestamp                         not null default now()
);