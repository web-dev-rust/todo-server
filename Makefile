db:
	docker run -i --rm --name auth-db -p 5432:5432 -e POSTGRES_USER=auth -e POSTGRES_PASSWORD=secret -d postgres

clear-db:
	docker ps -a | awk '{ print $1,$2 }' | grep postgres | awk '{print $1 }' | xargs -I {} docker stop {}

test: db
	sleep 2
	diesel setup
	diesel migration run
	cargo test --features "dbtest"
	diesel migration redo

run-local:
	cargo run --features "dbtest"

run:
	docker-compose up --build

down:
	docker-compose down