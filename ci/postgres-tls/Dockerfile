# The Compose stack's PostgreSQL, with TLS (spec §8.3a). The stack runs APP_ENV=production, where the service refuses
# a DB_URL without sslmode=require or stronger, so its database must offer TLS. This is the upstream image plus
# `openssl`, and an entrypoint that makes a self-signed certificate on first start. The service uses sslmode=require:
# the connection is encrypted, but the certificate is not checked, which is enough on the stack's own network and no
# more. A database elsewhere should have a certificate the service can verify (sslmode=verify-full).
FROM docker.io/library/postgres:18-alpine@sha256:0115267ab5a509022df9319c77be365cef335adb71556c75f60f5360ee247e71

RUN apk add --no-cache openssl

COPY --chmod=0755 tls-entrypoint.sh /usr/local/bin/tls-entrypoint.sh

ENTRYPOINT ["tls-entrypoint.sh"]
CMD ["postgres", \
     "-c", "ssl=on", \
     "-c", "ssl_cert_file=/var/lib/postgresql/tls/server.crt", \
     "-c", "ssl_key_file=/var/lib/postgresql/tls/server.key", \
     "-c", "ssl_min_protocol_version=TLSv1.3"]
