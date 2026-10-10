# Multi-stage container build for Project Portfolio Management Service.
#
# BUILD CONTEXT IS THE REPOSITORY ROOT. This crate depends on its sibling
# crate `project-portfolio-management-matcher-with-rust` by relative path
# (`../project-portfolio-management-matcher-with-rust`), so both must be in
# the context. Build from the repository root:
#
#   podman build -f project-portfolio-management-api-with-rust/Dockerfile \
#     -t project-portfolio-management-service .
#   # or: docker build -f project-portfolio-management-api-with-rust/Dockerfile -t ... .
#
# The two pinned git dependencies (`integrity-mac`, `authentication-verifier`
# from github.com/SixArm/main-x-service, locked by `rev` in Cargo.toml) are
# fetched by cargo during the build, which is why the builder needs `git` and
# `ca-certificates` and network access.
#
# Toolchain: the base image tag below must equal `channel` in the root
# rust-toolchain.toml; the first RUN step fails the build if they drift.

FROM docker.io/library/rust:1.98.1-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
        git \
        ca-certificates \
        pkg-config \
        libssl-dev \
        libpq-dev \
        perl \
        make \
        gcc \
    && rm -rf /var/lib/apt/lists/*

# Keep the builder in lock-step with the repository toolchain pin.
COPY rust-toolchain.toml /tmp/rust-toolchain.toml
RUN want="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' /tmp/rust-toolchain.toml)" \
    && have="$(rustc --version | cut -d' ' -f2)" \
    && echo "pinned toolchain ${want}, builder rustc ${have}" \
    && test "${want}" = "${have}"

WORKDIR /app

# Sibling path dependency (sources only; target/ is excluded by .dockerignore).
COPY project-portfolio-management-matcher-with-rust project-portfolio-management-matcher-with-rust

# This crate: manifests first, then the nested `migration` crate, sources,
# and benches. Cargo parses the whole manifest even for a --bin build, and
# Cargo.toml declares `[[bench]] name = "service_bench"`, so benches/ must exist.
COPY project-portfolio-management-api-with-rust/Cargo.toml project-portfolio-management-api-with-rust/Cargo.lock project-portfolio-management-api-with-rust/
COPY project-portfolio-management-api-with-rust/migration project-portfolio-management-api-with-rust/migration
COPY project-portfolio-management-api-with-rust/src project-portfolio-management-api-with-rust/src
COPY project-portfolio-management-api-with-rust/benches project-portfolio-management-api-with-rust/benches

WORKDIR /app/project-portfolio-management-api-with-rust

# Migrations are compiled into the binary, so the runtime stage needs no loose
# migration files; the same binary runs them (`... db migrate`). Default
# features only (the optional `fluvio` and `keycloak` features stay off).
RUN cargo build --release --locked --bin project-portfolio-management-service

# ---- runtime ---------------------------------------------------------------
FROM docker.io/library/debian:13-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        libpq5 \
        libssl3 \
        ca-certificates \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Non-root user.
RUN useradd --create-home --uid 1000 --shell /usr/sbin/nologin portfolio

WORKDIR /app

COPY --from=builder --chown=portfolio:portfolio \
    /app/project-portfolio-management-api-with-rust/target/release/project-portfolio-management-service \
    /app/project-portfolio-management-service

# loco reads config/<LOCO_ENV>.yaml at run time (it is not baked into the binary).
COPY --chown=portfolio:portfolio project-portfolio-management-api-with-rust/config ./config

# Tantivy full-text index (PROJECT_PORTFOLIO_MANAGEMENT_SEARCH_INDEX_PATH,
# default `data/search-index` relative to the working directory).
RUN mkdir -p /app/data/search-index && chown -R portfolio:portfolio /app

USER portfolio

EXPOSE 5150

# Loco's built-in /_health. Podman's default OCI image format drops
# HEALTHCHECK (build with `--format docker` to keep it); compose.yaml
# repeats an equivalent healthcheck so either format works.
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl --silent --fail http://localhost:5150/_health || exit 1

# config/production.yaml; requires DATABASE_URL. See /compose.yaml for the
# environment a real deployment needs (token-verifier keys, login enforcement).
ENV LOCO_ENV=production \
    PORT=5150

CMD ["/app/project-portfolio-management-service", "start"]
