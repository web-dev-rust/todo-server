db:
	docker run -i --rm --name auth-db -p 5432:5432 -e POSTGRES_USER=auth -e POSTGRES_PASSWORD=secret -d postgres &
	docker run -p 8000:8000 amazon/dynamodb-local

test:
	cargo test --features "dynamo"

run-local:
	cargo run --features "dynamo"

run:
	docker-compose up --build

down:
	docker-compose down