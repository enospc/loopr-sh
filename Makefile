BIN_DIR := bin
BIN := $(BIN_DIR)/loopr
ROOT_BIN := loopr
PKG := ./cmd/loopr
VERSION := $(shell git describe --tags --dirty --always 2>/dev/null || echo dev)
COMMIT := $(shell git rev-parse --short=12 HEAD 2>/dev/null || echo "")
DATE := $(shell date -u +%Y-%m-%dT%H:%M:%SZ)
LDFLAGS := -X 'loopr/internal/version.Version=$(VERSION)' -X 'loopr/internal/version.Commit=$(COMMIT)' -X 'loopr/internal/version.Date=$(DATE)'

.PHONY: build run fmt vet tidy clean

build:
	@command -v go >/dev/null 2>&1 || { echo "Go is required to build Loopr. Install Go 1.25+ and re-run make build."; exit 1; }
	mkdir -p $(BIN_DIR)
	go build -ldflags "$(LDFLAGS)" -o $(BIN) $(PKG)

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
