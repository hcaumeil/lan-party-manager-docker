alter table sessions
add created timestamp not null default now();
