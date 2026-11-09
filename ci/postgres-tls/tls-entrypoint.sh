#!/bin/sh
# Make the server's self-signed certificate once, on the data volume, then hand over to the upstream entrypoint.
# It lives beside the data (not in it), so a new certificate is only made for a new volume.
set -eu
dir=/var/lib/postgresql/tls
if [ ! -s "$dir/server.key" ] || [ ! -s "$dir/server.crt" ]; then
    mkdir -p "$dir"
    openssl req -new -x509 -nodes -days 3650 -newkey ec -pkeyopt ec_paramgen_curve:prime256v1 \
        -subj "/CN=postgres" -addext "subjectAltName=DNS:postgres,DNS:localhost" \
        -keyout "$dir/server.key" -out "$dir/server.crt" 2>/dev/null
fi
chown -R postgres:postgres "$dir"
chmod 0700 "$dir"
chmod 0600 "$dir/server.key"
exec docker-entrypoint.sh "$@"
