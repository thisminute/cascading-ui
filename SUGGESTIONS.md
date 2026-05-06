# Suggestions for Future Work

## Remaining Disabled Tests (3)

The `classes_1`, `classes_2`, and `classes_3` tests in `tests/data/dynamic.rs` require **class + mutable variable interaction** which is a harder problem than basic mutable variables.

### What's needed

When a mutable variable is referenced by elements that received it via class cascading, the Effect system needs to:

1. **Implement `EffectTarget::Class` handling** — currently the match in `initialize/dynamic.rs` only handles `EffectTarget::Element`. The `Class` variant needs to query all elements with that class selector and update them.

2. **Register Effects for class-targeted properties** — in `compiled_element`, Effects are registered for direct element properties. But when a property comes via class cascading, the Effect should target the class (so all instances update), not individual elements.

3. **Dynamic re-registration** — when a listener fires and creates new elements that reference mutable variables, those elements need to register themselves in the Effect system. Currently only static (pre-rendered) elements register Effects.

### Suggested approach

The commented-out `render_variables` function in `runtime/render.rs:85-115` had the right shape. Consider:

```rust
fn render_variable(state: &mut Vec<(Value, Vec<Effect>)>, id: usize, value: Value) {
    state[id].0 = value;
    for Effect { property, target } in &state[id].1 {
        match target {
            EffectTarget::Element(element) => render_property(element, property, state[id].0.clone()),
            EffectTarget::Class(class) => {
                let elements = document.get_elements_by_class_name(class);
                for i in 0..elements.length() {
                    let element = elements.item(i).unwrap().dyn_into::<HtmlElement>().unwrap();
                    render_property(&element, property, state[id].0.clone());
                }
            }
        }
    }
}
```

## Architecture Improvements

### Error handling in proc-macro
All `panic!()` calls in the compiler produce terrible error messages. Replace with `syn::Error` propagation:
- `value.rs:67` — "unable to render variable from ancestors" (should show the variable name and available scope)
- `cascade.rs:21` — self-cascade check (should never reach users but still)
- `html.rs:14` — unwrap on homepage (should say "no root page defined")

### id_gen atomic ordering
`id_gen.rs` uses `Ordering::Relaxed` for load + swap which is technically a race condition (two threads could get the same ID). For proc-macros this is fine since they're single-threaded, but if the code is ever used in multi-threaded compilation, use `fetch_add(1, Ordering::SeqCst)` instead.

### Variable resolution architecture
Currently, `render_values` resolves properties for elements, and `render_dynamic_subtree` resolves them for listener subtrees. These are two separate code paths doing similar work. Consider unifying into a single recursive pass that handles both.

### The cascade `or_insert` semantics
In `cascade.rs:30`, `or_insert(value)` means target properties take precedence over source. This is the correct CSS behavior (more-specific wins), but only because cascade is called source→target. If cascade order ever changes, this silently breaks. Consider adding an explicit comment or renaming.

### Unnecessary cloning
Many iterations use `.clone()` to avoid borrow checker issues (e.g., `self.groups[id].elements.clone()` before iterating while also mutating `self`). Most of these can't be easily fixed without restructuring, but some could use indices instead of cloning the entire Vec.

## Feature Roadmap

### Multi-page routing (next priority after mutable classes)
The `Page` struct has a `route: &'static str` field but only `"/"` is ever used. The `html()` function already generates a `HashMap<&str, String>` mapping routes to content. What's missing:
- Route-based directory structure parsing in `lib.rs`
- Client-side route switching in Wasm
- Link property generating `<a href="...">` with route paths

### Tooltip and Image properties
These parse correctly but `render_property` in `runtime/render.rs` returns `()` for both. Implementation:
- `tooltip` → set `title` attribute on element
- `image` → create `<img>` child or set `background-image` CSS

### Conditionals and loops
No syntax exists yet. Possible CUI syntax:
```
// Conditional
@if $condition {
    text: "shown";
}

// Loop
@each $item in $list {
    text: $item;
}
```
These would need new AST nodes, new Prefix variants, and compile-time vs runtime conditional logic.
