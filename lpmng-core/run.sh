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
export PUBLIC_DIR=./src/public/
export ROUTER_ADDRESS="http://127.0.0.1:2004"
export PORT=8000
export DATABASE_URL=postgres://corpau@localhost/lpmng
sqlx migrate run
cargo sqlx prepare
cargo r --release
