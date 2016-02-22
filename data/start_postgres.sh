#!/bin/bash
source pg_options.sh
if [ -z "$DATA_DIR" ]; then
    echo '$DATA_DIR must be set!'
    exit 1
fi
if [ -z "$LOG_DIR" ]; then
    echo '$LOG_DIR must be set!'
    exit 1
fi
mkdir -p "$LOG_DIR"
pg_ctl -D "$DATA_DIR" -l "$LOG_DIR/pg_log" start
