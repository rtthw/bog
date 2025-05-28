
> [!WARNING]
> This library is currently undergoing major revisions. Use at your own risk!

<details>
<summary>Table of Contents</summary>

- [Bog](#bog)
  - [Elevator Pitch](#elevator-pitch)
  - [Quickstart](#quickstart)
  - [Features](#features)
  - [Learn More](#learn-more)
  - [License](#license)

</details>

<!-- cargo-rdme start -->

<div align="center">

<br>[![Crate Badge]][Crate] [![Docs Badge]][Docs] [![License Badge]](./LICENSE)

</div>

# Bog

A set of tools designed to work as a replacement for (or addition to) Rust's standard library.

## Elevator Pitch

The primary goal of Bog is to have a set of interoperable tools that you can use from the early stages of any project all the way to its final release. Things useful for (almost) anyone doing (almost) anything.

There is no "one size fits all" abstraction. That's why everthing in Bog is divorced from everthing else. Rendering is not tied to window management. User interfacing is not tied to layout. UI elements are not tied to the user interface. You can pick and choose what fits your specific use case best.

I want to provide a set of tools you can use to make your project *your project*. This is not a framework, it's a toolkit.

## Quickstart

*Source: [quickstart.rs](examples/quickstart.rs)*

```rust
use bog::prelude::*;

fn main() -> Result<()> {
    run_app(QuickstartApp)
}

struct QuickstartApp;

impl AppHandler for QuickstartApp {
    fn window_desc(&self) -> WindowDescriptor {
        WindowDescriptor {
            title: "Quickstart",
            ..Default::default()
        }
    }
}

impl View for QuickstartApp {
    fn build(&mut self, layout_map: &mut LayoutMap) -> Model<Self> {
        let mut theme = Theme::default();
        let style = StyleClass::new(&mut theme, Styling {
            bg_color: Some(Color::new(43, 43, 53, 255)),
            text_height: Some(Unit::Em(4.0)),
            text_slant: Some(TextSlant::Italic),
            ..Default::default()
        });

        Model::new(
            Element::new()
                .layout(Layout::default()
                    .align_items_center()
                    .justify_content_center())
                .child(
                    static_paragraph("Hello, world!").style(style)
                ),
            layout_map,
            theme,
        )
    }
}
```

## Features

By default, all features are enabled and available. You can choose which ones you want by setting `default-features` to `false` in your `Cargo.toml`, and then enabling the ones you want:

```toml
bog = { version = "*", default-features = false, features = ["window", "render"] }
```

- `app`, an easy way to create cross-platform applications.
- `layout`, for CSS-style layout management.
- `render`, for rendering to surfaces with the GPU.
- `window`, for connecting to the platform's windowing system.

## Learn More

- [Notes on the project's architecture](./docs/ARCHITECTURE.md)
- [Reference sheet for various interfaces](./docs/REFERENCE.md)

## License

[MIT](./LICENSE)

[Crate]: https://crates.io/crates/bog
[Crate Badge]: https://img.shields.io/crates/v/bog?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44
[Docs Badge]: https://img.shields.io/docsrs/bog?logo=rust&style=flat-square&logoColor=E05D44
[Docs]: https://docs.rs/bog
[License Badge]: https://img.shields.io/crates/l/bog?style=flat-square&color=1370D3
