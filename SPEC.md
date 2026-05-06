# CUI Language Specification

CUI is a web language with CSS-like syntax. You write structure, style, and behavior in one unified notation. The compiler handles the mapping to HTML, CSS, and WebAssembly.

## Core Concepts

A CUI program is a tree of **blocks**. Every block is one of three kinds:

| Block kind | Syntax | Meaning |
|-----------|--------|---------|
| **Instance** | `name { }` | Create a concrete element named `name` here |
| **Class** | `.name { }` | Define how all `name` instances in this scope should look/behave |
| **Listener** | `?event { }` | When `event` fires on this element, apply these changes |

That's it. Everything inside a block is either a **property** or a nested **block**.

## Blocks in Detail

### Instances

An instance creates a visible element in the output.

```
greeting {
    text: "hello world";
}
```

The name is the identity. If a class `.greeting` exists in any ancestor scope, its properties cascade into this instance automatically. No explicit "apply class" syntax needed — the name IS the connection.

Instances render in document order. Nesting creates parent-child relationships:

```
page {
    header {
        title { text: "My Site"; }
    }
    content {
        paragraph { text: "Welcome."; }
    }
}
```

### Classes

A class defines properties and structure that apply to all same-named instances within its scope.

```
.card {
    background: "#fff";
    border-radius: "8px";
    padding: "16px";

    .card_title {
        font-weight: "700";
        font-size: "1.2rem";
    }
}
```

Key rules:
- **Scope**: A class applies to instances in the scope where it's defined, and all nested scopes beneath it
- **Name matching**: Instance `foo` automatically inherits from class `.foo` if `.foo` is visible in any ancestor scope
- **Priority**: Instance properties override class properties (more specific wins)
- **Order-independent**: `.card` can be defined before or after `card` instances at the same scope level (hoisting)
- **Nesting**: Classes can contain sub-classes, instances, and listeners — all of which cascade into matching instances

The cascade mechanism:

```
// Class defined at root scope
.button {
    background: "blue";
    color: "white";
}

page {
    // This instance picks up .button's properties because
    // .button is defined in an ancestor scope (root)
    button {
        text: "Click me";
    }

    // Another instance, same class, different content
    button {
        text: "Cancel";
    }
}
```

Classes can nest classes and instances to define component structure:

```
.card {
    padding: "16px";

    // Defines how card_title looks INSIDE a card
    .card_title {
        font-weight: "bold";
    }

    // Defines how card_body looks INSIDE a card
    .card_body {
        color: "#444";
    }
}

// Usage — the instance structure mirrors the class structure
card {
    card_title { text: "Hello"; }
    card_body { text: "World"; }
}
```

### Listeners

A listener fires when a browser event occurs. Its contents are applied when the event triggers.

```
button {
    text: "Click me";
    color: "blue";

    ?click {
        text: "Clicked!";
        color: "green";
    }
}
```

Listeners can contain properties, instances, classes, and nested listeners:

```
?click {
    text: "now click again";

    ?click {
        text: "done";
    }
}
```

Supported events: `click`, `blur`, `focus`, `mouseover`, `mouseenter`, `mouseleave`, `mouseout`.

## Properties

A property is a `name: value;` pair. The property name determines what it does:

### CSS Properties

Any standard CSS property works directly. The value is a quoted string.

```
color: "blue";
font-size: "1.2rem";
background: "#f0f0f0";
border-radius: "8px";
margin: "16px 0";
display: "flex";
```

### CUI Properties

These are CUI-specific and control structure/behavior rather than style:

| Property | What it does | Status |
|----------|-------------|--------|
| `text:` | Set the text content of this element | Working |
| `link:` | Make this element a link (renders as `<a>`) | Working |
| `title:` | Set the page title (only meaningful at root scope) | Working |
| `tooltip:` | Set hover tooltip text | Parsed, not yet compiled |
| `image:` | Set an image source | Parsed, not yet compiled |

### Future Properties

The property system is designed to grow. Any new capability (forms, media, animation, routing) would be added as a new property keyword. The developer's mental model stays the same: `name: value;`.

## Variables

Variables store values that can be referenced by properties.

```
$greeting: "hello";
text: $greeting;
```

### Static Variables

A variable used only in one scope resolves at build time. No runtime cost.

```
.card {
    $padding: "16px";
    padding: $padding;
}
```

Variables cascade through classes just like properties:

```
.card {
    $color: "blue";
}

card {
    color: $color;    // resolves to "blue"
}
```

### Mutable Variables

A variable becomes mutable when the same name appears in a listener on the same element. The compiler detects this automatically.

```
$label: "Click me";
text: $label;
?click {
    $label: "Clicked!";
}
```

The compiler promotes this to reactive state: `$label` gets backed by a Wasm state slot, and all properties referencing it update when it changes.

**Current limitation**: The variable definition and listener must be on the same element. Cross-element mutation (variable on parent, listener on child) is not yet supported.

## File Organization

CUI supports multi-file projects. Place `.cui` files in a `cui/` directory alongside your project. They are automatically loaded and merged into the root scope.

```
project/
  cui/
    styles.cui      // Global classes
    hero.cui        // Hero component classes
    card.cui        // Card component classes
  app/
    src/lib.rs      // Page structure (instances)
```

Classes defined in `.cui` files are available to all instances, just as if they were written at the top of the main file. This lets you separate concerns:

- **`.cui` files** define appearance (classes with properties)
- **`lib.rs`** defines structure (instances with content)

## Compilation Layers

The compiler analyzes the program and assigns every piece of content to one of three layers:

| Layer | When it runs | What goes here |
|-------|-------------|---------------|
| **Static** | Build time | Content with no listeners or mutable variables. Baked directly into the HTML string. Zero runtime cost. |
| **Initialize** | Page load (once) | Content that exists in the initial DOM but needs event listeners wired up. Runs once via `wasm_bindgen(start)`. |
| **Reactive** | On events | Content behind listeners with mutable variables. Managed by Wasm runtime state. |

The developer never specifies layers. The compiler infers them from the code. A page with no listeners compiles to pure static HTML with no JavaScript or Wasm at all.

## Property Reference (Quick)

### CUI-specific
```
text: "content";              // element text content
link: "https://example.com";  // make element a link
title: "Page Title";          // page <title> (root only)
tooltip: "hover text";        // tooltip (not yet compiled)
image: "url";                 // image (not yet compiled)
```

### Common CSS (all ~170 standard properties supported)
```
// Layout
display: "flex";
flex-direction: "column";
gap: "16px";
margin: "0 auto";
padding: "16px";
width: "100%";
max-width: "800px";
position: "relative";

// Typography
font-family: "Inter, sans-serif";
font-size: "1rem";
font-weight: "700";
line-height: "1.5";
text-align: "center";
color: "#333";

// Visual
background: "#f0f0f0";
border: "1px solid #ddd";
border-radius: "8px";
box-shadow: "0 2px 4px rgba(0,0,0,0.1)";
opacity: "0.9";

// Interactive
cursor: "pointer";
transition: "all 0.2s ease";
```

### Events
```
?click { }
?focus { }
?blur { }
?mouseover { }
?mouseenter { }
?mouseleave { }
?mouseout { }
```

### Variables
```
$name: "value";     // define
property: $name;    // use
```
