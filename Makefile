db:
	docker run -p 8000:8000 amazon/dynamodb-local

test:
	cargo test --features "dynamo"