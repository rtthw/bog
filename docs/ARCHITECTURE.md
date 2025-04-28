


# Architecture

There are 3 categories that define the various crates you'll find in this repository:

1. **Foundation**, crates that provide functionality used in the other two categories.
2. **Interface**, crates that provide interfaces to the computer's the core systems.
2. **Abstraction**, crates that abstract over some common uses for the computer's core systems.

The Bog crate itself (found in `src`) does not fit into any of these categories and merely serves as a way to glue them together.



---

## Foundation

### bog_alloc

*TODO*

### bog_color

*Color type definitions and conversions.*

### bog_env

*Environment type definitions and functions.*

### bog_event

*Event type definitions.*

### bog_math

*Various mathematical types and functions.*



---

## Interface

### bog_files

*Interfaces for filesystems.*

### bog_window

*Interfaces for window managers, display servers, and compositors.*



---

## Abstraction

### bog_fonts

*Font type definitions and parsing.*

### bog_layout

*Planar layout types and functionality.*

### bog_render

*Abstractions for graphics programming.*



---

## Patterns

### Handler Indirection

The most common high-level pattern you'll see used throughout Bog is something I like to call "trait indirection". It's really **just a way of composing trait objects without having them explicitly "rely" on one another**.

Take, for example, the `LayoutHandler` type:

```rust
trait LayoutHandler {
    fn on_layout(&mut self, args: Args);
}
```

This is the "indirection type" used by the `LayoutTree`, which is used by the `UserInterface` type to handle the laying-out of its elements. But what happens when the `UserInterface` type wants its own indirection type, like the `InterfaceHandler`?

It could do something like this:

```rust
trait InterfaceHandler: LayoutHandler { /* BODY */ }
```

This wouldn't normally be a problem as long as the types that implement `InterfaceHandler` don't need access to any other functionality/state other than what has been defined by the `LayoutHandler`. But what if the `InterfaceHandler` 's `on_layout` function needs access to something like the current mouse position? The `LayoutHandler` doesn't even know there is a mouse at all, nor should it.

```rust
struct UserInterface {
    layout_tree: LayoutTree,
    mouse_pos: Vec2,
}

impl UserInterface {
    fn on_resize(&mut self, handler: &mut impl InterfaceHandler) {
        // The `do_layout` function calls `on_layout`. `LayoutTree` expects something that
        // implements `LayoutHandler`.
        self.layout_tree.do_layout(&mut Proxy {
            handler,
            mouse_pos: self.mouse_pos,
        });
    }
}

trait InterfaceHandler {
    fn on_layout_with_mouse_pos(&mut self, args: Args, mouse_pos: Vec2);
}

struct Proxy<'a> {
    handler: &'a mut dyn InterfaceHandler,
    mouse_pos: Vec2,
}

impl<'a> LayoutHandler for Proxy<'a> {
    fn on_layout(&mut self, args: Args) {
        self.handler.on_layout_with_mouse_pos(args, self.mouse_pos);
    }
}
```
