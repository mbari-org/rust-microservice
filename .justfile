set dotenv-load := true

_:
	#!/usr/bin/env bash
	if [ $(which fzf) ] && [ -x $(which fzf) ]; then
		just --choose
	else
		just --list --unsorted
	fi

# List recipes
list:
	@just --list --unsorted

# Run dockerized postgres
run-postgres:
	docker run --rm \
		--name postgres -e POSTGRES_PASSWORD=docker \
		-p 5432:5432 -v /tmp/postgress:/var/lib/postgresql/data \
		postgres

# Create/migrate database
create-database:
	(cd news-migrations && cargo run)

# Run the service
run-service:
	RUST_LOG=info cargo run --bin news-service

# Run dockerized psql
psql:
	docker exec -it postgres sh -c "PGPASSWORD=docker /usr/bin/psql -h 127.0.0.1 -U postgres -d postgres"
