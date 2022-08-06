#!/bin/bash
set -e;

while true;
do
    ./run gen > /tmp/json;
    jq < /tmp/json || exit 1;
done
