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
