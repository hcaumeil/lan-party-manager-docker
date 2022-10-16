create table if not exists sessions
(
    id          uuid    default gen_random_uuid() not null primary key,
    lastname    text                              not null,
    firstname   text                              not null,
    user_id     uuid,
    internet    boolean default false             not null
);