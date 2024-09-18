Commands to remember - all executed from root directory.

**Generate new migration**
```
sea-orm-cli migrate generate --migration-dir ./quoted_db_migration create_character_table
```

**Deploy migration**
```
cargo run --bin quoted_db_migration -- up
```

**Generate entities from schema**
```
sea-orm-cli generate entity -o ./quoted_db_entity/src
```

**Execute seeder**
```
cargo run --bin quoted_db_seeder
```