# rust-aws-lambda-playground
A place for me to experiment with Rust AWS Lambdas

## Prerequisites

- `cargo`
- `cargo-lambda`
- `zig`
- `docker`
- `sqlx-cli`

## How-to

- Setup the database locally:
  - `docker compose up -d`
  - `cd migrations`
  - `echo "DATABASE_URL=postgres://root_usr:root_pwd@localhost:5432/root_dbn" >> .env`
  - `sqlx migrate run --source sql`
  - `sqlx migrate info --source sql`
- Test the RestAPI lambda locally:
  - `docker compose up -d`
  - `echo "DATABASE_URL=postgres://root_usr:root_pwd@localhost:5432/root_dbn" >> .env`
  - `cargo lambda watch` or `cargo lambda watch --release`
  - The RestAPI will then be available at [localhost:9000](http://localhost:9000)
- Create a new migration:
  - `cd migrations`
  - `echo "DATABASE_URL=postgres://root_usr:root_pwd@localhost:5432/root_dbn"` >> .env
  - `sqlx migrate add --source sql migration_name`, where you replace `migration_name` with your migration's name
  - Start editing the new `.sql` file created in `migrations/sql`

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
