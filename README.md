<details>
<summary>Table of Contents</summary>

- [Bog](#bog)
  - [Usage](#usage)
  - [Features](#features)
  - [License](#license)

</details>

<!-- cargo-rdme start -->

<div align="center">

<br>[![Crate Badge]][Crate] [![Docs Badge]][Docs] [![License Badge]](./LICENSE)

</div>

# Bog

The highly-modular abstraction standard.

## Usage

The recommended design pattern to be used with this library is the basic type definition wrapper...

```rust
type Position = bog::xy::Xy<u16>;
```

And implementing custom features should be done through extension traits...

```rust
trait PositionImpl {
    fn column(&self) -> u16;
}

impl PositionImpl for Position {
    fn column(&self) -> u16 {
        self.x
    }
}
```

With the following results...

```rust
let pos_a = Position::new(1, 2);

assert_eq!(pos_a.column(), 1);
```

## Features

By default, all features are enabled, but you can disable this by setting `default-features` to false and manually selecting which features you want in your project's Cargo.toml:

```text
bog = { version = "*", default-features = false, features = ["rect"] }
```

- `all`, all of the following features.
- `color`, the `Color` type for working with visuals.
- `easing`, a set of functions that apply easings to inputs.
- `rect`, a rectangle abstraction.
- `xy`, an X-Y coordinate value.
- `xyz`, an X-Y-Z coordinate value.

## License

[MIT](./LICENSE)

[Crate]: https://crates.io/crates/bog
[Crate Badge]: https://img.shields.io/crates/v/bog?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44
[Docs Badge]: https://img.shields.io/docsrs/bog?logo=rust&style=flat-square&logoColor=E05D44
[Docs]: https://docs.rs/bog
[License Badge]: https://img.shields.io/crates/l/bog?style=flat-square&color=1370D3
