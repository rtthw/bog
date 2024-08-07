<details>
<summary>Table of Contents</summary>

- [Bog](#ratatui)
  - [Usage](#usage)
  - [Features](#features)

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
type Position = bog::Xy<u16>;
```

And implementing custom features should be done through extension traits...

```rust
trait PositionImpl {
    fn custom_x_getter(&self) -> u16;
}

impl PositionImpl for Position {
    fn custom_x_getter(&self) -> u16 {
        self.x
    }
}
```

With the following results...

```rust
let pos_a = Position::new(1, 2);

assert_eq!(pos_a.custom_x_getter(), 1);
```

## Features

| Name      | Rect | X-Y | X-Y-Z |
| :--       | :-:  | :-: | :-:   |
| `default` |✔|✔| |
| `all`     |✔|✔|✔|
| `rect`    |✔| | |
| `xy`      | |✔| |
| `xyz`     | | |✔|

[Crate]: https://crates.io/crates/bog
[Crate Badge]: https://img.shields.io/crates/v/bog?logo=rust&style=flat-square&logoColor=E05D44&color=E05D44
[Docs Badge]: https://img.shields.io/docsrs/bog?logo=rust&style=flat-square&logoColor=E05D44
[Docs]: https://docs.rs/bog
[License Badge]: https://img.shields.io/crates/l/bog?style=flat-square&color=1370D3
