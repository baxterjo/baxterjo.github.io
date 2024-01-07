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
pre_commit: build docs/404.html
	git add docs/
	cargo test


.PHONY: build
	dx build --release


.PHONY: serve_local
serve_local:
	npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch & \
	dx serve --hot-reload

