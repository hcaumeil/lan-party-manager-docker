create extension if not exists pgcrypto;

create table if not exists users
(
    id          uuid    default gen_random_uuid() not null primary key,
    username    text                              not null unique,
    firstname   text                              not null,
    lastname    text                              not null,
    email       text                              not null unique,
    password    text                              not null,
    phone       text                              not null unique,
    role        text                              not null,
    is_allowed  boolean default false             not null
);