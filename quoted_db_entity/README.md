# Quoted DB Entities

This is a library containing only the entities that represent the database tables. 
All of these entities are generated via the [sea-orm-cli](https://crates.io/crates/sea-orm-cli), 
which is run from the workspace root.

```
sea-orm-cli generate entity -o ./quoted_db_entity/src
```

After running this command, any new entities will be added as a module to [src/](./src/), 
however, we need to manually update [`src/lib.rs`](./src/lib.rs) and declare the module 
as public, by adding `pub mod <entity-module-name>;`. This allows it to be consumed by 
other crates in the workspace.