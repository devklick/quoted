# Quoted

An app that allows creation and retrieval of your favorite quotes from your
favorite TV shows. 

## Stack

### Backend

#### API

This is serverless vercel function that uses the [rust runtime](https://github.com/vercel-community/rust). See the following crates for more on this:

- [`quoted_api`](./quoted_api/)
- [`quoted_api_models`](./quoted_api_models/)

#### Database

The API is hooked up to a postgres instance in Vercel. Interaction with the database is performed via [SeaORM](https://www.sea-ql.org/SeaORM/), including migrations. See the following crates for more on this:

- [`quoted_db`](./quoted_db/)
- [`quoted_db_migration`](./quoted_db_migration/)
- [`quoted_db_entity`](./quoted_db_entity/)

#### Database Seeder

Currently, database population is done via a seeder application. This is done manually. See the [`quoted_db_seeder`](./quoted_db_seeder/) crate for more on this.

### Front End

There are two front ends:
- [`quoted-ui`](./quoted_ui/) - Web user interface
- [`quoted_cli`](./quoted_cli/) Command-line user interface