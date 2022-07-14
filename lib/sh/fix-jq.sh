#!/bin/bash

[ "$UID" -eq 0 ] || exec sudo bash "$0" "$@"

apt-get update && apt-get install -y jq

echo "You may need to restart tool to pick-up changes"
