#!/bin/sh
set -e 

iptables -P FORWARD DROP
iptables -N LPMNG
iptables -A FORWARD -j LPMNG 
export ROUTER_ADDRESS="0.0.0.0:2004"
lpmng-router

