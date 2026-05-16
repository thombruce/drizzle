# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

Drizzle CSS — a classless-first CSS framework. Source CSS modules in `css/` are bundled and minified by a small Rust binary (`src/main.rs`) that uses the [Lightning CSS](https://lightningcss.dev/) crate. Demo HTML pages in `demo/` link to the built artifacts in `dist/`.

## Commands

```sh
cargo build --release            # compile bundler
cargo run --release              # build CSS — emits dist/drizzle.css + drizzle.min.css
./target/release/drizzle-build   # rerun bundler without recompile

python3 -m http.server 8765      # preview demos at http://localhost:8765/demo/index.html
                                 # (file:// works too but Chrome may block it under Playwright)
```

No test suite, no linter. CSS validity is enforced at bundle time — Lightning CSS surfaces parse errors and fails the build.

## Architecture

### Two-layer build

`src/main.rs` is the only Rust code. It runs `lightningcss::bundler::Bundler` against `css/drizzle.css`, inlining every `@import`, then prints the stylesheet twice — once unminified, once minified — with browser targets pinned to Chrome 95 / Firefox 90 / Safari 15 / Edge 95. Modern CSS syntax (e.g. `color-mix`, nesting) is preserved when the target supports it and down-leveled otherwise. The bundler must be **re-run after any change to `css/**`** — demos link to `dist/drizzle.css`, not the source modules.

### Source CSS layering — order is load-bearing

`css/drizzle.css` is the single entry point. It `@import`s every module in a deliberate order:

```
tokens → dark → reset → base → typography → lists → tables → forms → media → utilities → palette
```

This order is the specificity strategy. Every selector in the framework is either bare-element (no class) or single-class (e.g. `.card`, `.bg-red`). All single-class selectors have equal specificity, so **source order is what decides who wins**. Two rules matter most:

1. **`palette.css` MUST be imported last.** It defines `.bg-red`, `.bg-blue`, etc. If anything that sets `background` (like `.card` in `utilities.css`) comes after palette, it will silently override the hue on `<div class="card bg-red">`. This bug shipped in the very first build; the fix was the import-order swap. Don't undo it.
2. Tokens come first because every later rule references `var(--...)` from them.

### Classless-first contract

Every visible semantic HTML element is styled by a **bare element selector**, never via a required class. Adding a class is opt-in enhancement. When editing `base.css`, `typography.css`, `lists.css`, `tables.css`, `forms.css`, or `media.css`, keep selectors classless. Reserve class selectors for `palette.css` (color modifiers) and `utilities.css` (layout/component helpers).

### Theming via CSS variables

All visual values live in `css/tokens.css` as custom properties on `:root` — palette hues, semantic aliases (`--color-error`, `--color-warning`, `--color-success`, `--color-info`), surface/text/border colors, font families, type scale, weights, gutters, borders, radii, shadows, layout widths. Downstream consumers retheme by overriding a single var; no Sass, no build step on their side.

### Dark mode — two parallel paths

`css/dark.css` declares the same dark overrides in two places:
- `@media (prefers-color-scheme: dark) { :root:not(.light) { ... } }` — auto-respect OS, but `<html class="light">` opts out.
- `:root.dark { ... }` — explicit class override regardless of OS.

When adding new themed values, update both blocks together or dark mode will diverge from class-toggled dark.

### Bundler output is gitignored

`dist/` and `target/` are in `.gitignore`. CI / consumers run `cargo run --release` to regenerate. Do not commit built CSS.
