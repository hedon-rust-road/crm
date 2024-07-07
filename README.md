# CRM

## Generate test data

### Run migration to create `user_stats` table

```bash
sqlx migrate run --target-version 20240706163534
```

### Run script to generate test data

```bash
cd user-stat
cargo run --example gen --release
```

### Run migration to create indexes for `user_stats` table

```bash
sqlx migrate run
```
