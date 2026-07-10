# Portfolio Website

## Setup

Styling uses [Tailwind CSS](https://tailwindcss.com/) via the standalone CLI binary (no Node/npm required). Run `just setup-dev` to download it to `bin/tailwindcss` (macOS/Linux, arm64/x64). `just build` and `just serve-local` both compile `input.css` to `public/css/tailwind.css` automatically.

## Deploying

Pushes to `main` trigger a GitHub Actions workflow (`.github/workflows/deploy.yml`) that runs `just build` and publishes the `dist/` output straight to GitHub Pages. Build artifacts are never committed to the repo — `just build` is only for previewing a production build locally.

## Adding Content

To add a new page to the site in the `experience` or `projects` tree in the website, add a new Markdown file to the corresponding location in the `site_content` directory. The file should start with a `+++`-delimited TOML frontmatter block (`title`, `thumbnail`, `description`, `date_added`, `priority_level`) followed by the page content in Markdown. See `site_content/content_template.md` for an example.

### Images and Thumbnails

Thumbnails should have a 3:2 aspect ratio (or they won't show up well in the card galleries on mobile)

The location of assets should be in the `public/img` directory and follow the same file structure as the `site_content` directory.
