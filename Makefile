# UTILS
.git/hooks/pre-commit: hooks/pre-commit
	cp -R hooks/pre-commit .git/hooks/pre-commit

docs/404.html: docs/index.html
	cp -R docs/index.html docs/404.html
	git add docs/404.html


# JOBS
.PHONY: setup_dev
setup_dev: .git/hooks/pre-commit

.PHONY: pre_commit
pre_commit: test

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	rm -rf docs/*
	cargo clean

.PHONY: all
all: clean build
	mv docs/public/* docs
	cp docs/index.html docs/404.html

.PHONY: build
build:
	dx bundle --release


.PHONY: serve_local
serve_local:
	dx serve --hot-reload

