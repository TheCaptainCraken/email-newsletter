#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "  cargo install sqlx-cli --no-default-features --features rustls,postgres to install it."
    exit 1
fi

# If a custom password hasn't been set, set it to user = postgres, password = password
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Same for other settings for the db.
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=6666}"
DB_HOST="${POSTGRES_HOST:=localhost}"


if [[ -z "${SKIP_DOCKER}" ]]
then
    #Launch the db using docker
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
fi

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}

export DATABASE_URL
export PGPASSWORD="${DB_PASSWORD}"

# Keep pinging Postgres until it's ready to accept commands
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    echo >&2 "Postgres is still unavailable - sleeping"
    sleep 1
done

echo "Database is alive, running migrations!"

sqlx database create
sqlx migrate run

echo "Migrations complete."

echo "Everything is set and ready!"