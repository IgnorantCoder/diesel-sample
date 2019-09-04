#!/usr/bin/env sh

export DATABASE_URL=postgres://postgres@db:5432/sample
sleep 7

cd /tmp/sample
diesel setup
echo 'start application'
./target/debug/diesel-sample