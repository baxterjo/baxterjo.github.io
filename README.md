# Portfolio Website

## Setup

Styling uses [Tailwind CSS](https://tailwindcss.com/) via the standalone CLI binary (no Node/npm required). Download it to `bin/tailwindcss`:

```sh
curl -sL "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-<platform>" -o bin/tailwindcss
chmod +x bin/tailwindcss
```

Replace `<platform>` per the [Tailwind CLI install docs](https://tailwindcss.com/docs/installation/tailwind-cli) (e.g. `macos-arm64`, `linux-x64`, `windows-x64.exe`). `just build` and `just serve_local` both compile `input.css` to `public/css/tailwind.css` automatically.

## Adding Content

To add a new page to the site in the `about`, `experience`, or `projects` tree in the website, add a new content directory to the corresponding location in the `site_content` directory.

### Images and Thumbnails

Thumbnails should have a 3:2 aspect ratio (or they won't show up well in the card galleries on mobile)

The location of assets should be in the `public/img` directory and follow the same file structure as the `site_content` directory.