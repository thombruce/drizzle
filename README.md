# Drizzle CSS

A classless-first CSS framework driven by CSS variables, bundled with the Rust [Lightning CSS](https://lightningcss.dev/) crate.

- **Classless by default** — bare semantic HTML renders well with zero classes or IDs.
- **Variables everywhere** — every color, size, weight, gutter, border, and radius is a CSS custom property. Override one to retheme.
- **Dark mode** — auto via `prefers-color-scheme`, with `.dark` / `.light` overrides for manual toggling.
- **Tiny** — single file, no runtime, no JS, no dependencies for consumers.

## Quick start

```html
<link rel="stylesheet" href="dist/drizzle.css">
<body>
  <h1>Hello</h1>
  <p>This page is styled. No classes were used.</p>
  <button>Go</button>
</body>
```

## Download (prebuilt)

Each tagged release publishes `drizzle.css` and `drizzle.min.css` as GitHub release assets. Pin a version or pull the latest:

```sh
# Latest
curl -O https://github.com/thombruce/drizzle/releases/latest/download/drizzle.min.css

# Pinned
curl -O https://github.com/thombruce/drizzle/releases/download/v0.1.0/drizzle.min.css
```

## Use in a Rust project

Add to `Cargo.toml`:

```toml
[dependencies]
drizzle-css = "0.1"
```

The CSS is compiled at build time and embedded as `&'static str` constants:

```rust
// Write to your output directory
std::fs::write(output_dir.join("drizzle.css"), drizzle_css::CSS_MIN)?;
```

Then reference it from generated HTML:

```html
<link rel="stylesheet" href="drizzle.css">
```

Both `drizzle_css::CSS` (readable) and `drizzle_css::CSS_MIN` (minified) are available.

## Build

Requires Rust (stable). The bundler uses Lightning CSS to inline `@import`s, transform modern syntax for target browsers, and emit both readable and minified outputs.

```sh
cargo run --release
```

Produces:

- `dist/drizzle.css` — bundled, readable
- `dist/drizzle.min.css` — bundled, minified

## Demos

Open any file in `demo/` in a browser:

| File | Showcases |
|---|---|
| `demo/index.html` | Overview + theme toggle |
| `demo/typography.html` | Headings, prose, code, blockquote, inline elements |
| `demo/forms.html` | Every input type, validation, buttons |
| `demo/tables.html` | Simple + captioned + zebra tables |
| `demo/lists.html` | ul, ol, dl, nested, task lists |
| `demo/palette.html` | 8 hues + 4 semantic aliases + alerts |
| `demo/components.html` | details, figure, dialog, cards, buttons |

## Customising

Override any variable on `:root` (or any scope):

```css
:root {
  --color-link: #ff6b6b;
  --radius: 0;
  --font-family-base: "Inter", system-ui, sans-serif;
}
```

Dark-theme values are single-sourced as `--dark-*` tokens. Override one var and both the OS-driven and `.dark`-toggled paths pick it up:

```css
:root {
  --color-link:      #ff6b6b;   /* light theme */
  --dark-color-link: #ff9b9b;   /* dark theme  */
}
```

### Variable reference (summary)

| Group | Variables |
|---|---|
| Palette | `--color-{red,orange,yellow,green,blue,indigo,violet,pink}` |
| Semantic | `--color-{error,warning,success,info}` |
| Surface | `--color-{bg,surface,surface-2,text,muted,border,link,link-hover}` |
| Dark | `--dark-color-{bg,surface,surface-2,text,muted,border,link,link-hover}`, `--dark-shadow{,-sm,-lg}` |
| Type | `--font-family-{sans,mono,serif,base}`, `--font-size-{xs,sm,base,lg,xl,h1..h6}` |
| Weight | `--font-weight-{light,normal,medium,bold,heading}` |
| Spacing | `--gutter-{xs,sm,md,lg,xl,2xl}` |
| Border | `--border-width-{thin,,thick}`, `--border-style`, `--border` |
| Radius | `--radius-{sm,,lg,full}` |
| Shadow | `--shadow-{sm,,lg}` |
| Layout | `--container-width`, `--content-width` |

See `css/tokens.css` for the full list and defaults.

## Project layout

```
css/             # source modules (entry: drizzle.css)
src/lib.rs       # library — exposes CSS and CSS_MIN constants
src/main.rs      # binary — bundles CSS to dist/
build.rs         # compiles CSS at consumer build time
demo/            # demo HTML pages
dist/            # build output (gitignored)
Cargo.toml
```

## License

MIT.
