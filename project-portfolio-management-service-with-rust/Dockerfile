# Multi-stage container build for Project Portfolio Management Service.
#
# BUILD CONTEXT IS THE REPOSITORY ROOT (the root rust-toolchain.toml is read
# from it). The matcher, `authentication-verifier` and `integrity-mac` come from crates.io.
# Build from the repository root:
#
#   podman build -f project-portfolio-management-service-with-rust/Dockerfile \
#     -t project-portfolio-management-service .
#
# Every dependency now comes from crates.io (no git dependencies), so the build
# needs network access to the registry and `ca-certificates`.
#
# Version: pass `--build-arg VERSION=<x.y.z>` (the release workflow does, from the tag) and the build fails
# unless it equals the crate's `version` in Cargo.toml, and the image is labelled with it
# (org.opencontainers.image.version). Without the argument no version label is set. `REVISION` (the git
# commit) is optional and sets the revision label.
#
# Toolchain: the base image tag below must equal `channel` in the root
# rust-toolchain.toml; the first RUN step fails the build if they drift.

FROM docker.io/library/rust:1.99.1-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
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

# This crate: manifests first, then the nested `migration` crate, sources,
# and benches. Cargo parses the whole manifest even for a --bin build, and
# Cargo.toml declares `[[bench]] name = "service_bench"`, so benches/ must exist.
COPY project-portfolio-management-service-with-rust/Cargo.toml project-portfolio-management-service-with-rust/Cargo.lock project-portfolio-management-service-with-rust/
COPY project-portfolio-management-service-with-rust/migration project-portfolio-management-service-with-rust/migration
COPY project-portfolio-management-service-with-rust/src project-portfolio-management-service-with-rust/src
COPY project-portfolio-management-service-with-rust/benches project-portfolio-management-service-with-rust/benches

WORKDIR /app/project-portfolio-management-service-with-rust

# The crate's version, read from Cargo.toml; a VERSION build argument must agree with it.
ARG VERSION=""
RUN crate="$(sed -n 's/^version *= *"\(.*\)"/\1/p' Cargo.toml | head -1)" \
    && echo "crate version ${crate}, VERSION argument '${VERSION}'" \
    && { [ -z "${VERSION}" ] || [ "${VERSION}" = "${crate}" ]; }

# Migrations are compiled into the binary, so the runtime stage needs no loose
# migration files; the same binary runs them (`... db migrate`). Default
# features plus CARGO_FEATURES below.
# Cargo features on top of the default (PASETO) set. `keycloak` is on so one image can verify
# tokens from a Keycloak realm, including one that brokers Microsoft Entra ID
# (operations/sign-in-with-entra.md); it is inert unless KEYCLOAK_* is configured. Add more with
# `--build-arg CARGO_FEATURES="keycloak fluvio"`; empty it to build the default set only.
ARG CARGO_FEATURES="keycloak"
RUN cargo build --release --locked --features "${CARGO_FEATURES}" --bin project-portfolio-management

# ---- runtime ---------------------------------------------------------------
FROM docker.io/library/debian:13-slim

ARG VERSION=""
ARG REVISION=""
LABEL org.opencontainers.image.version="${VERSION}" \
      org.opencontainers.image.title="project-portfolio-management" \
      org.opencontainers.image.description="Project portfolio management service (loco.rs REST API, PostgreSQL)" \
      org.opencontainers.image.source="https://github.com/project-portfolio-management/project-portfolio-management" \
      org.opencontainers.image.licenses="MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only" \
      org.opencontainers.image.revision="${REVISION}"

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
    /app/project-portfolio-management-service-with-rust/target/release/project-portfolio-management \
    /app/project-portfolio-management

# loco reads config/<LOCO_ENV>.yaml at run time (it is not baked into the binary).
COPY --chown=portfolio:portfolio project-portfolio-management-service-with-rust/config ./config

# Tantivy full-text index (PROJECT_PORTFOLIO_MANAGEMENT_SEARCH_INDEX_PATH,
# default `data/search-index` relative to the working directory) and the
# artifact store (PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR). Both are meant to
# be mounted volumes (see /compose.yaml); the directories exist so a fresh
# named volume inherits the non-root ownership.
RUN mkdir -p /app/data/search-index /app/data/artifacts && chown -R portfolio:portfolio /app

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

CMD ["/app/project-portfolio-management", "start"]
