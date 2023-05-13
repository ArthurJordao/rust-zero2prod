#!/bin/bash

LOCKFILE=$PGDATA/postmaster.pid
if [ ! -f "$LOCKFILE" ]; then
  pg_ctl -o "-p $PGPORT -k $PGDATA" start 
fi
sqlx database create
sqlx migrate run
