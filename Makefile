CARGO = cargo
TARGET = release

.PHONY: help
help:
	@echo ""
	@echo "Available make commands:"
	@echo "  make run      - Run the project in --release mode"
	@echo "  make client   - Run the demo client script in"
	@echo "  make build    - Build the project in --release mode"
	@echo "  make test     - Run all unit and integration tests, none for now"
	@echo "  make fmt      - Format the code using rustfmt"
	@echo "  make clean    - Remove the target directory"
	@echo "  make help     - Show this help message"
	@echo ""

.PHONY: run
run:
	$(CARGO) run --$(TARGET) 

.PHONY: client
client:
	bash bash/client.sh

.PHONY: build
build:
	$(CARGO) build --$(TARGET) 

.PHONY: test
test:
	$(CARGO) test

.PHONY: fmt
fmt:
	$(CARGO) fmt


.PHONY: clean
clean:
	$(CARGO) clean

