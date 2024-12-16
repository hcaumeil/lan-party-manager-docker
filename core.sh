#!/bin/sh
set -e 
sleep 10
psql postgres://corpau@10.82.0.3/lpmng -f /bin/init.sql
export ADMIN_KEY=toto
export CLIENT_KEY=titi
export PUBLIC_DIR=/bin/public/
export ROUTER_ADDRESS="http://10.82.0.2:2004"
export PORT=8000
export DATABASE_URL=postgres://corpau@10.82.0.3/lpmng
lpmng-core

