CARGO ?= cargo

.PHONY: build test run web bench docker hash-tests test-original build-native submission-verify submission-demo

build:
	$(CARGO) build --release --bin mnemonist

build-native:
	npm run build:native

test:
	$(CARGO) test

test-original:
	npm run test:original:all-ported

run:
	$(CARGO) run --release --bin mnemonist -- --help

web:
	$(CARGO) run --release --bin mnemonist -- --web

bench:
	$(CARGO) run --release --bin bench

docker:
	docker build -t mnemonist-port .

submission-verify:
	npm run verify:submission

submission-demo:
	npm run demo:submission

hash-tests:
ifeq ($(OS),Windows_NT)
	@powershell -NoProfile -File scripts/hash-tests.ps1
else
	@bash scripts/hash-tests.sh
endif
