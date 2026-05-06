# Session: 2026-05-05

## What was done

### Bug fixes
1. **id_gen.rs OOB** — `SYMBOLS_2[digit]` → `SYMBOLS_2[digit - SYMBOLS_1.len()]`. Prevented panic at 2809+ class IDs.
2. **cascade.rs variable shadowing** — `for source_id in` → `for element_id in`, fixed unwrap on potentially-None name.
3. **value.rs discarded render** — Variable definitions now actually update after rendering (result was thrown away before).
4. **dynamic.rs type mismatch** — `compiled_dynamic_value` was returning `state[mutable_id]` (a tuple) instead of `state[mutable_id].0.clone()` (the Value). `compiled_dynamic_properties` was double-wrapping in `Value::String(...)`.
5. **Effect propagation** — `EffectTarget::Class` now handled in the generated code. Effects use current state value instead of literal.
6. **Unresolved variables in listeners** — Added `render_dynamic_subtree()` to resolve variable references in listener subtrees that weren't going through `render_element()`.
7. **html.rs hardcoded import** — `import init from './cui/cui_app_template.js'` was hardcoded. Now derives from `CARGO_PKG_NAME` env var so each CUI app loads the correct Wasm module.
8. **Submodule SSH URLs** — Changed `.gitmodules` in both `cascading-ui` and `cui-tools` from SSH (`git@github.com:`) to HTTPS (`https://github.com/`) so CI can resolve dependencies.

### Modernization
- Edition 2015 → 2021 (all `use` paths updated to `crate::`)
- ChromeDriver updated to match Chrome 147

### Tests
- Re-enabled 3 of 6 disabled tests: `from_listener`, `into_listener`, `between_listeners`
- Added 4 new tests: `multiple_clicks`, `variable_with_css`, `variable_in_child_element`, `listener_creates_element_text`
- Final count: **39 tests passing** (was 32)
- Fixed `between_listeners` assertion to include child element in inner_html

### Still disabled (3 tests)
`classes_1`, `classes_2`, `classes_3` — require full class+mutable variable Effect system. See SUGGESTIONS.md.

## Key architectural insight
The 3 layers (static/initialize/runtime) are determined by `listener_scope`:
- `is_compiled()` = `listener_scope.is_none()` → Layer 1 (static HTML)
- Has listener_scope, immutable vars → Layer 2 (initialize once)
- Mutable vars (same name across different scopes) → Layer 3 (reactive)

Mutable detection happens in `cascade.rs` when `virtual_=true` and both source and target have the same variable name.

**Critical limitation discovered**: Mutable variable detection only works when the variable definition and listener modification are on the SAME element. Cross-element mutation (variable on parent, listener on child/sibling) fails because the cascade only checks the direct parent, not the entire ancestor chain. This is the same root cause as the disabled `classes_1/2/3` tests.

### Parser improvements (for the website)
- Added hyphenated CSS property parsing (`font-family`, `border-radius`, etc.)
  - Changed `Property.property` from `Ident` to `String` in AST
  - Parser now consumes `ident-ident` sequences joined by `-`
  - `peek_property` changed from `Ident + :` to `Ident + !Brace`
- Added ~30 modern CSS properties to the phf set (border-radius, flexbox, grid, transforms, transitions, etc.)

### Website (cascading-ui.net)
- Created at `../cascading-ui.net/` as a workspace with `app` (CUI source) and `server` (actix-web)
- Builds to HTML + Wasm via `wasm-pack build --target web`
- Has interactive demo with mutable variable (click button changes its own text)
- The website IS the demo — built entirely in CUI syntax
- **Deployed live** at https://cascading-ui.net via GitHub Pages
- CI/CD pipeline: push to main → wasm-pack build → deploy to Pages
- SSL auto-provisioned by GitHub Pages

### Deploy pipeline fixes
- Fixed HTML output path in CI: `target/html/index.html` (workspace root, not `app/target/html/`)
- Fixed Wasm import path: now uses `CARGO_PKG_NAME` instead of hardcoded `cui_app_template`
- Fixed all SSH submodule URLs to HTTPS for CI compatibility

## Next priorities
1. **Cross-element mutable variable detection** — cascade needs to check ancestor chain, not just direct parent. This unblocks:
   - The remaining 3 disabled tests (`classes_1/2/3`)
   - Richer interactive demos on the website
2. Tooltip/image codegen
3. Multi-page routing
