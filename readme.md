# Quoted

An app that allows creation and retrieval of your favorite quotes from your
favorite TV shows. 

## Stack

### Backend

#### API

This is serverless vercel function that uses the [rust runtime](https://github.com/vercel-community/rust). See the following crates for more on this:

- [quoted_api](./quoted_api/)
- [quoted_api_models](./quoted_api_models/)

#### Database

The API is hooked up to a postgres instance in Vercel. Interaction with the database is performed via [SeaORM](https://www.sea-ql.org/SeaORM/), including migrations. See the following crates for more on this:

- [`quoted_db`](./quoted_db/)
- [`quoted_db_migration`](./quoted_db_migration/)
- [`quoted_db_entity`](./quoted_db_entity/)

#### Database Seeder

Currently, database population is done via a seeder application. This is done manually. See the [`quoted_db_seeder`](./quoted_db_seeder/) crate for more on this.

### Front End

At the time of writing this, the only that is intended for users to interact with 
is the [`quoted_cli`](./quoted_cli/) (it's not really a front end, but I'll class it 
as that because it's what users will interact with). 

This is a command line application that allows fetching quotes from your favorite TV shows. 
It may also be expanded in the future to test your knowledge of a given show with a "who said it"
type of quiz, where the user will be presented with a series of quotes and need to guess the character, 
the episode etc.