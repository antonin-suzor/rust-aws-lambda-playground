# rust-aws-lambda-playground
A place for me to experiment with Rust AWS Lambdas

## Prerequisites

- `cargo`
- `cargo-lambda`
- `zig`

## How-to

- Test the lambda locally:
  - `cargo lambda watch` or `cargo lambda watch --release`
  - `curl -v localhost:9000`
- Deploy manually:
  - `cargo lambda deploy --binary-name rust_aws_lambda lambda_name --enable-function-url`
