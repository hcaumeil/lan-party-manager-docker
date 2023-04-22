#!/bin/bash
export NODE_OPTIONS=--openssl-legacy-provider
if [ ! "$(ls -A src/public)" ]; then
    cd ../lpmng-front
    yarn run nuxt generate
    cp -R dist/ -T ../lpmng-core/src/public
    cd -
fi
export ADMIN_KEY=toto
export CLIENT_KEY=titi
export ROUTER_ADDRESS="http://127.0.0.1:8080"
export DATABASE_URL=postgres://corpau@localhost/lpmng
sqlx migrate run
cargo sqlx prepare
cargo r --release
