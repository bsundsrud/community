#!/bin/bash
source pg_options.sh
if [ -z "$DATA_DIR" ]; then
    echo '$DATA_DIR must be set!'
    exit 1
fi
pg_ctl -D "$DATA_DIR" stop
