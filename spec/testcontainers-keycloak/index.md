# Automated integration testing with Testcontainers and Keycloak

For backend services (like Spring Boot or .NET), spin up a real Keycloak
instance inside a container during your test lifecycle. This provides full
realism without depending on a permanent external server.

- Add the `testcontainers-keycloak` dependency to your project build file.
- Configure your test runner (like JUnit 5) to launch a dynamic Keycloak
  container before running tests.
- Import a pre-configured realm JSON file containing your test users and
  clients so you don't have to build them programmatically.
- Inject the dynamic Keycloak issuer URI into your application configuration
  properties before hitting secured endpoints with a requested JWT token.

## Container runtime in this workspace

This workspace uses Podman, not Docker (see `rust-loco-stack.md`). Testcontainers
can drive Podman through its Docker-compatible socket; fully qualify the image
name (for example `quay.io/keycloak/keycloak`) because Podman refuses short
names.
