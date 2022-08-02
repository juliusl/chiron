#!/bin/bash

python3 -c "from azure.cli.core import get_default_cli; get_default_cli().invoke(['acr', 'login', '--expose-token', '--name', 'obddemo', '--output', 'tsv', '--query', 'accessToken'])"
