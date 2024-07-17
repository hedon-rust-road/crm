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

## Some useful commands

### Get indexes size

```bash
select pg_size_pretty (pg_indexes_size('user_stats')) size;
```

### Get data size

```bash
select pg_size_pretty (pg_relation_size('user_stats')) size;
```

### Create export table

```sql
create table export_user_stats as select * from user_stats limit 100;
```
