#!/bin/bash

set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=logs}
DB_PORT=${POSTGRES_PORT:=5432}
INSTANCE_NAME=${INSTANCE_NAME:=logdb}

if [[ -z "${REMOVE_DB}" ]]; then
    docker stop ${INSTANCE_NAME}
    docker rm ${INSTANCE_NAME}
fi

docker run \
    --name ${INSTANCE_NAME} \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    echo >&2 "Postgres is still unavailable sleeping ..."
    sleep 1
done

echo >&2 "Postgres is up and running on port  ${DB_PORT} ..."

sqlx database create
sqlx migrate run

echo >&2 "Postgresql has been migrate, ready!"
