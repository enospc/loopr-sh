BIN_DIR := bin
BIN := $(BIN_DIR)/loopr
PKG := ./cmd/loopr

.PHONY: build run fmt vet tidy clean

build:
	mkdir -p $(BIN_DIR)
	go build -o $(BIN) $(PKG)

run:
	go run $(PKG)

fmt:
	gofmt -w cmd internal *.go

vet:
	go vet ./...

tidy:
	go mod tidy

clean:
	rm -rf $(BIN_DIR)
