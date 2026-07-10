# UTILS
install-tailwind:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -x bin/tailwindcss ]; then
        echo "tailwindcss already installed at bin/tailwindcss"
        exit 0
    fi
    case "$(uname -s)" in
        Darwin) os=macos ;;
        Linux) os=linux ;;
        *) echo "Unsupported OS: $(uname -s)" >&2; exit 1 ;;
    esac
    case "$(uname -m)" in
        arm64|aarch64) arch=arm64 ;;
        x86_64|amd64) arch=x64 ;;
        *) echo "Unsupported architecture: $(uname -m)" >&2; exit 1 ;;
    esac
    mkdir -p bin
    curl -fsSL "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-${os}-${arch}" -o bin/tailwindcss
    chmod +x bin/tailwindcss

# JOBS
setup-dev: install-tailwind

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

serve-local: tailwind
    ./bin/tailwindcss -i ./input.css -o ./public/css/tailwind.css --watch &
    dx serve --hot-reload
