#!/bin/sh
mkdir -p ./build
g++ server.cpp -Wall -Wextra -Og -g -o ./build/server
g++ client.cpp -Wall -Wextra -Og -g -o ./build/client
