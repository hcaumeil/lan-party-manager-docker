#!/bin/sh
set -e 

ip r del default
ip r add default via 10.82.0.2 dev eth0
sleep 15
curl -H "Content-Type: application/json" -X POST -d '{"username": "test_user", "firstname": "test", "lastname": "test", "email": "test@example.com", "password": "test", "phone": "0707070707", "role": "user", "is_allowed": false}' http://10.82.0.4:8000/api/users
result=$(curl -H "Content-Type: application/json" -X POST -d '{"login": "test_user", "password": "test"}' http://10.82.0.4:8000/api/login)
token=$(echo $result | jq -r '.biscuit')
uid=$(echo $result | jq -r '.user_id')
curl -H "Content-Type: application/json" -H "Authorization: Bearer $token" -X POST -d "{\"user_id\": \"$uid\", \"ip4\": \"\", \"internet\": false,\"date_time\":\"2024-12-14T14:33:05+0100\"}" http://10.82.0.4:8000/api/sessions
ping 1.1.1.1 

