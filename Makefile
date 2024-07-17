export:
	@pg_dump -h localhost -p 5432 -U postgres --table=export_user_stats --data-only --column-inserts -d stats > ./user_stats/fixtures/data.sql