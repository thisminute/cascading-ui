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

## Design Proposals (from cascading-ui.net work)

### LocalStorage for Dark Mode Persistence

CUI currently has no concept of browser APIs. The dark mode toggle resets on page reload.

**Option A — `persist:` property on variables (recommended):**
```
let $bg: "white" persist: "theme-bg";
```
At page load, the Wasm initialize phase checks `localStorage.getItem("theme-bg")`. If found, uses that value instead of the default. On every assignment, calls `localStorage.setItem("theme-bg", newValue)`.

**Option B — `storage` block type:**
```
storage {
    $bg: "theme-bg";
    $fg: "theme-fg";
    $tile: "theme-tile";
}
```
Maps variables to localStorage keys. Clean separation of concerns but introduces a new block type.

Option A is preferred — it's per-variable, minimal syntax, and fits the "properties on things" philosophy.

**Implementation notes:**
- Affects `analyze.rs` (parse `persist:` on `let` declarations)
- Affects `wasm/initialize/` (read localStorage before first render)
- Affects `wasm/runtime/` (write localStorage on variable assignment)
- The `persist` key should be optional — most variables don't need it

### Resetting State Between Tutorial Steps

When navigating between lessons, interactive state (clicked buttons, toggled checkboxes) persists in the background. Options:

**Option A — Manual reset in navigation handlers (works today):**
```
l5_next {
    ?click {
        $show_l5: "none";
        $show_l6: "block";
        $l6_status: "nothing checked";  // reset lesson 6 state
    }
}
```
Verbose but requires no language changes.

**Option B — `?init` listener (new language feature):**
```
lesson_6 {
    ?init {
        $l6_status: "nothing checked";
    }
}
```
Fires when the element becomes visible. The compiler would generate Wasm that watches the display variable and runs the init block on transition from `none` → `block`. Fits the existing listener pattern (`?click`, `?blur`, etc.).

**Option C — `reset-on-hide` property:**
```
lesson_6 {
    display: $show_l6;
    reset-on-hide: "true";
}
```
When `display` transitions to `"none"`, all descendant variables reset to their `let` defaults. Requires the compiler to track initial values and generate reset logic.

Option B (`?init`) is the most CUI-idiomatic — it's just another listener type.

### URL-Based Routing

Current state: SPA with display variables (`$show_home`, `$show_l1`, etc.). True routing needs direct URL access and browser history.

**Proposed syntax:**
```
/home {
    // homepage content
}

/tutorial {
    // tutorial shell with sidebar

    /lessons/1 {
        // lesson 1 content
    }
    /lessons/2 {
        // lesson 2 content
    }
}
```

**Implementation path:**
1. **Parse**: `/path { }` blocks become route blocks — new AST node type alongside Element, Class, Listener
2. **Compile**: Generate a Wasm router that listens to `popstate` events
3. **Initialize**: Read `window.location.pathname` on load, show the matching route
4. **Navigation**: A `navigate:` property (or reuse `link:` for internal routes) that calls `history.pushState()` instead of setting display variables
5. **Nested routes**: `/tutorial` shows the shell, `/tutorial/lessons/1` shows shell + lesson content

The `Page` struct already has a `route: &'static str` field and `html()` generates `HashMap<&str, String>`. The plumbing exists — it just needs to be connected to client-side navigation.

**Incremental approach**: Start with flat routes, add nesting later.

### Responsive / Mobile Views

CUI doesn't support `@media` queries. Options:

**Option A — `?media` listener (recommended):**
```
sidebar {
    width: "200px";
    ?media "(max-width: 768px)" {
        display: "none";
    }
}
```
Fits the existing listener pattern. The compiler generates a `matchMedia` listener in Wasm, just like `?click` generates a click handler. Consistent mental model.

**Option B — `@media` blocks:**
```
page {
    sidebar { width: "200px"; }

    @media "(max-width: 768px)" {
        sidebar { display: "none"; }
    }
}
```
More CSS-like but introduces a new block type that doesn't fit the Instance/Class/Listener model.

**Option C — Viewport variables:**
```
let $sidebar_display: "block" media: "(max-width: 768px)" "none";
```
Variable defaults change based on viewport. Compile-time only.

Option A (`?media`) is preferred — it uses the existing listener infrastructure, requires no new block types, and enables reactive viewport-based behavior.
