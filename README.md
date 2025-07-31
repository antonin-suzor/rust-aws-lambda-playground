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
  - get a postgresql instance running and available in a `.env` file at the project root with `DATABASE_URL`
  - `cargo lambda watch` or `cargo lambda watch --release`
  - `curl -v localhost:9000`

## Extra Documentation

- https://www.cargo-lambda.info/
- https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html
- https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples/
- https://crates.io/crates/sqlx
- https://crates.io/crates/sqlx-cli
