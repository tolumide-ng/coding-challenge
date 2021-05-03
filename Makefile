.PHONY: help
help:
	@echo "----------------Project Commands ---------------"
	@echo "make build: build the entire project"
	@echo "make test: runs the test for the entire project"
	@echo "------------------------------------------------"


.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY:
test:
	cargo test