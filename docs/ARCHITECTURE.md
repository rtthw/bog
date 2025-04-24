


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
