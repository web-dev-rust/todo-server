db:
	docker run -p 8000:8000 amazon/dynamodb-local

test:
	cargo test --features "dynamo"

run-local:
	cargo run --features "dynamo"

run:
	docker-compose up --build

down:
	docker-compose down