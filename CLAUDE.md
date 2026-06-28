# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

Drizzle CSS — a classless-first CSS framework. Source CSS modules in `css/` are bundled and minified at compile time by `build.rs` (using the [Lightning CSS](https://lightningcss.dev/) crate) and embedded into the `drizzle_css` library crate as `&'static str` constants. The `drizzle-build` binary (`src/main.rs`) just writes those constants to `dist/`. Demo HTML pages in `demo/` link to the built artifacts in `dist/`.

## Commands

```sh
cargo build --release            # bundle CSS (build.rs) into the library
cargo run --release              # build + emit dist/drizzle.css + drizzle.min.css
./target/release/drizzle-build   # re-emit dist/ from the embedded constants

python3 -m http.server 8765      # preview demos at http://localhost:8765/demo/index.html
                                 # (file:// works too but Chrome may block it under Playwright)
```

No test suite, no linter. CSS validity is enforced at bundle time — Lightning CSS surfaces parse errors and fails the build.

## Architecture

### Build pipeline

The bundling lives in **`build.rs`**, which runs `lightningcss::bundler::Bundler` against `css/drizzle.css` at compile time, inlining every `@import`, then prints the stylesheet twice — once unminified, once minified — with browser targets pinned to Chrome 95 / Firefox 90 / Safari 15 / Edge 95. Modern CSS syntax (e.g. `color-mix`, nesting, cascade layers) is preserved when the target supports it and down-leveled otherwise. Output is written to `OUT_DIR` and embedded by `src/lib.rs` as the `CSS` / `CSS_MIN` constants. `src/main.rs` (the `drizzle-build` bin) only writes those constants out to `dist/`; it does **not** re-bundle, so the pinned targets exist in exactly one place. A change to `css/**` requires a rebuild (`cargo run --release`) — demos link to `dist/drizzle.css`, not the source modules.

### Source CSS layering — cascade layers decide who wins

`css/drizzle.css` is the single entry point. It declares an explicit `@layer` order, then `@import`s every module into its matching layer:

```
tokens → dark → reset → base → typography → lists → tables → forms → media → utilities → print → palette
```

**Cascade layers — not specificity, not source order — are the strategy.** Every selector is bare-element or single-class (e.g. `.card`, `.bg-red`), so specificity is mostly flat; the `@layer` order in `drizzle.css:7` is what resolves conflicts. A later layer beats an earlier one. Two rules matter most:

1. **`palette.css` MUST stay in the last layer.** It defines `.bg-red`, `.bg-blue`, etc. If anything that sets `background` (like `.card` in `utilities.css`) landed in a later layer, it would silently override the hue on `<div class="card bg-red">`. Keep palette last in both the `@layer` declaration and the import list.
2. Tokens come first because every later rule references `var(--...)` from them.

Note: because the import list and the `@layer` declaration are kept in the same order, source order and layer order agree — but layer order is the one that's load-bearing. `!important` inverts layer precedence (earlier layers win), which is why `print.css` uses `!important` to force print styles regardless of its layer position.

### Classless-first contract

Every visible semantic HTML element is styled by a **bare element selector**, never via a required class. Adding a class is opt-in enhancement. When editing `base.css`, `typography.css`, `lists.css`, `tables.css`, `forms.css`, or `media.css`, keep selectors classless. Reserve class selectors for `palette.css` (color modifiers) and `utilities.css` (layout/component helpers).

### Theming via CSS variables

All visual values live in `css/tokens.css` as custom properties on `:root` — palette hues, semantic aliases (`--color-error`, `--color-warning`, `--color-success`, `--color-info`), surface/text/border colors, font families, type scale, weights, gutters, borders, radii, shadows, layout widths. Downstream consumers retheme by overriding a single var; no Sass, no build step on their side.

### Dark mode — two parallel paths

`css/dark.css` declares the same dark overrides in two trigger blocks:
- `@media (prefers-color-scheme: dark) { :root:not(.light) { ... } }` — auto-respect OS, but `<html class="light">` opts out.
- `:root.dark { ... }` — explicit class override regardless of OS.

Both blocks map active vars (`--color-bg`, `--shadow`, etc.) to defaults declared once in `css/tokens.css` as `--dark-*` tokens. End users retheme dark mode by overriding a single `--dark-*` var. When adding a new themed value, add the `--dark-*` default to `tokens.css` and the `--color-x: var(--dark-color-x)` mapping to **both** trigger blocks — values are now single-sourced, but var **names** must stay in sync between the two triggers.

### Bundler output is gitignored

`dist/` and `target/` are in `.gitignore`. CI / consumers run `cargo run --release` to regenerate. Do not commit built CSS.
