# The Compose stack's PostgreSQL, with TLS (spec §8.3a). The stack runs APP_ENV=production, where the service refuses
# a DATABASE_URL without sslmode=require or stronger, so its database must offer TLS. This is the upstream image plus
# `openssl`, and an entrypoint that makes a self-signed certificate on first start. The service uses sslmode=require:
# the connection is encrypted, but the certificate is not checked, which is enough on the stack's own network and no
# more. A database elsewhere should have a certificate the service can verify (sslmode=verify-full).
FROM docker.io/library/postgres:18-alpine@sha256:10a5c69a83035ab5239f5620d708520e8b28f5aa1f2b20143e6024532edadfb5

RUN apk add --no-cache openssl

COPY --chmod=0755 tls-entrypoint.sh /usr/local/bin/tls-entrypoint.sh

ENTRYPOINT ["tls-entrypoint.sh"]
CMD ["postgres", \
     "-c", "ssl=on", \
     "-c", "ssl_cert_file=/var/lib/postgresql/tls/server.crt", \
     "-c", "ssl_key_file=/var/lib/postgresql/tls/server.key", \
     "-c", "ssl_min_protocol_version=TLSv1.3"]
