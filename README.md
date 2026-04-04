## Hive Console / Grafbase Composition

This service exposes [Hive Console External Composition](https://the-guild.dev/graphql/hive/docs/schema-registry/self-hosting/external-composition) service that uses [Grafbase's composition library](https://github.com/grafbase/grafbase/tree/main/crates/graphql-composition) for production a Federation Supergraph. 

The need for such service is because Grafbase's artifacts are not aligned and not compliant with the Federation spec, as it composes and adds code on top of the core Federation directives.

## Service API 

The service exposes the following endpoints on port `4000`:

- `GET /health` - as healthcheck / readiness check
- `POST /compose` - endpoint for composing the supergraph 

## Known Issues / Caveats

- Due to `panic`s in Grafbase's legacy code (`cynic` parser), this library omits all descriptions in order to avoid calling parser code that is not maintained.
- [Grafbase's composition is not 100% compliant with Federation spec](https://the-guild.dev/graphql/hive/federation-gateway-audit)

## Running Locally

To Run locally, ensure you have:

- Rust 1.94 installed 
- `cargo install patch-crate`

Then, use the following script to run it locally: 

```
cargo run
```

## Building as image

A `Dockerfile` is present in the repository, to build it as a service. Use the following to build the image: 

```
docker build -t grafbase-external-composition:MY_TAG_NAME . 
```

Or, to build for a specific arch:

```
docker buildx build --platform linux/amd64 -t grafbase-external-composition:MY_TAG_NAME .
```
