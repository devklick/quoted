# Quoted DB Seeder

This app is responsible for seeding data into the Quoted DB. 
It reads data from CSV* and adds it to the database if it does not already exist.

## Usage

### .env file

A `.env` file should be present in the workspace root directory and should contain 
a `DATABASE_URL` variable with the database connection string.

From the root directory, run the app with cargo:

```
cargo run --bin quoted_db_seeder
```

## CSV Files

The CSV files can be found in the [data directory](./data/). At present, there
are two types of CSV file:

### Shows

This file lists all shows, their seasons and episodes. It is required before
any quotes can be added, as quotes need to be linked to a show, season and episode.

At the moment, this file is populated manually using data from online 
(for example, from Wikipedia).

### Quotes

This file lists quotes to be added to the database. Each quote must specify 
the name show, season and episode it belongs to, as well as the character being quoted.

At the moment, this file is populated manually by watching the show and entering
the details of the quote to be uploaded.

## How the data is used

*These CSV files are embedded into the application at build time, then added to the database
at runtime. The inserts are idempotent, so if the show, season, episode, character or quote already exists, 
it'll just be ignored. 

In regards to the CSV data being embedded into the app - obviously this means
that as more shows and quotes get added, the application size will grow. Since there's
no intention for this application to be distributed, the increasing size shouldn't matter too much, 
other than it potentially increasing the build times. 

There potential to avoid the Shows CSV file and instead write a scraper for Wikipedia. This would
reduce size of the built application and reduce the manual overhead when adding data.