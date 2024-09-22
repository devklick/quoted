# Quoted DB Seeder

This app is responsible for seeding data into the Quoted DB. 
It reads shows, seasons, episodes, and quotes from Google Sheets and 
idempotently inserts them into the database.

## Usage

### .env file

A `.env` file should be present in the workspace root directory and should contain 
a `DATABASE_URL` variable with the database connection string.

The following optional environment variables can also be specified, or they can 
be provided as CLI arguments:
- `QUOTED_DB_SEEDER__GOOGLE_SERVICE_ACCOUNT_KEY_PATH`
- `QUOTED_DB_SEEDER__SHOWS_GOOGLE_SHEET_ID`
- `QUOTED_DB_SEEDER__QUOTES_GOOGLE_SHEET_ID`

### Running the app

From the root directory, run the app with cargo:

```
cargo run --bin quoted_db_seeder
```

## Google Sheets

The data is initially input into google sheets by a user, and the seeder pulls the
data from these sheets and inserts them into the database.

### Shows

This file lists all shows, their seasons and episodes. It is required before
any quotes can be added, as quotes need to be linked to a show, season and episode.

The column structure of this spreadsheet is:
- ShowName
- SeasonNo
- EpisodeNo
- EpisodeName (optional)

At the moment, this file is populated manually using data from online 
(for example, from Wikipedia).

### Quotes

This file lists quotes to be added to the database. Each quote must specify 
the name show, season and episode it belongs to, as well as the character being quoted.

The column structure of this spreadsheet is:
- ShowName
- SeasonNo
- EpisodeNo
- CharacterName
- QuoteText

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

## Wikipedia data used

The following Wikipedia resources have been used.

- [List of Family Guy episodes](https://en.wikipedia.org/wiki/List_of_Family_Guy_episodes)