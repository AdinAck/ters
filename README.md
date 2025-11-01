# ters

Generate getters and setters procedurally.

Annotate fields with `#[get]` to generate a getter method.
```rust
use ters::ters;

#[ters]
struct Foo {
    a: i32,
    #[get]
    b: bool,
}

fn getters() {
    let foo = Foo { a: 42, b: true };
    assert_eq!(foo.b(), &true);
}
```

Annotate fields with `#[set]` to generate a setter method.
```rust
use ters::ters;

#[ters]
struct Foo {
    #[set]
    a: i32,
    b: bool,
}

fn setters() {
    let mut foo = Foo { a: 42, b: true };
    foo.set_a(31);
}
```

Annotate fields with `#[get]` and `#[set]` to generate both a getter and a setter method.
```rust
use ters::ters;

#[ters]
struct Foo {
    #[get]
    #[set]
    a: i32,
    b: bool,
}

fn getters_and_setters() {
    let mut foo = Foo { a: 42, b: true };
    assert_eq!(foo.a(), &42);
    foo.set_a(31);
    assert_eq!(foo.a(), &31);
}
```

Unannotated fields will not have generated getters or setters.
```rust
use ters::ters;
#[ters]
struct Foo {
    a: i32,
    #[get]
    b: bool,
}
fn getters_not_generated() {
    let foo = Foo { a: 42, b: true };
    assert_eq!(foo.a(), &42); // this method doesn't exist
}
```
