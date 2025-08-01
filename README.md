# rust-aws-lambda-playground
A place for me to experiment with Rust AWS Lambdas

## Prerequisites

- `cargo`
- `cargo-lambda`
- `zig`
- `postgresql`
- `sqlx-cli`

## How-to

- Test the lambda locally:
  - `docker compose up -d`
  - `echo "DATABASE_URL=postgres://root_usr:root_pwd@localhost:5432/root_dbn" >> .env
  - `cargo lambda watch` or `cargo lambda watch --release`
  - `curl -v localhost:9000`

### Example `.env` file

```
#DATABASE_URL=postgres://root_usr:root_pwd@localhost:5432/root_dbn
POSTGRES_USR=root_usr
POSTGRES_PWD=root_pwd
POSTGRES_EDP=
POSTGRES_PRT=
POSTGRES_DBN=root_dbn
POSTGRES_CRT=

RUST_LOG=debug
```

## External Documentation

- https://www.cargo-lambda.info/
- https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html
- https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples/
- https://crates.io/crates/sqlx
- https://crates.io/crates/sqlx-cli
