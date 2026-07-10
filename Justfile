# UTILS
setup_hooks:
    cp -R hooks/pre-commit .git/hooks/pre-commit

# JOBS
setup_dev: setup_hooks

pre_commit: test

test:
    cargo test

clean:
    rm -rf docs/*
    cargo clean

all: clean build
    mv docs/public/* docs
    cp docs/index.html docs/404.html
    git add docs/404.html

tailwind:
    ./bin/tailwindcss -i ./input.css -o ./public/css/tailwind.css

build: tailwind
    dx bundle --release

serve_local: tailwind
    ./bin/tailwindcss -i ./input.css -o ./public/css/tailwind.css --watch &
    dx serve --hot-reload
