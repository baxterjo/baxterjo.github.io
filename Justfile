# UTILS
setup_hooks:
    cp -R hooks/pre-commit .git/hooks/pre-commit

# JOBS
setup_dev: setup_hooks

pre_commit: test

test:
    cargo test

clean:
    rm -rf dist/*
    cargo clean

tailwind:
    ./bin/tailwindcss -i ./input.css -o ./public/css/tailwind.css

build: clean tailwind
    dx bundle --release
    mv dist/public/* dist
    cp dist/index.html dist/404.html

serve_local: tailwind
    ./bin/tailwindcss -i ./input.css -o ./public/css/tailwind.css --watch &
    dx serve --hot-reload
