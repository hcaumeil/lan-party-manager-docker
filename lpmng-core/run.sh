#!/bin/bash
if [ ! "$(ls -A src/public)" ]; then
    cd ../lpmng-front
    yarn run nuxt generate
    cp -R dist/ -T ../lpmng-core/src/public
    cd -
fi
export ADMIN_KEY=toto
export CLIENT_KEY=titi
export DATABASE_URL=postgres://corpau@localhost/lpmng
sqlx migrate run
cargo sqlx prepare
cargo r
