#!/bin/sh
# wait-for-postgres.sh
# This is a script that makes sure that the db is up and running before running the backend
# This will permit the migration command to work
set -e

host="$1"
shift
cmd="$@"

until PGPASSWORD=$POSTGRES_PASSWORD psql -h "$host" -U "postgres" -c '\\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up - executing command"

diesel migration run

>&2 echo "Migration done, ready to use the database"

exec $cmd
