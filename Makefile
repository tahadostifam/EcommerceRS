dev:
	cargo run -p restapi

recreate-postgres:
	docker-compose -f ./docker-compose.devel.yml down postgres
	docker-compose -f ./docker-compose.devel.yml up -d postgres

migrate-dev:
	diesel migration run --database-url "postgresql://admin:admin@localhost:5432/default"