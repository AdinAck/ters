#![doc = include_str!("docs.md")]
#![no_std]

pub use ters_macros::ters;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters() {
        #[ters]
        struct Foo {
            #[allow(unused)]
            a: i32,
            #[get(deref)]
            b: bool,
        }

        let foo = Foo { a: 42, b: true };
        assert_eq!(foo.b(), true);
    }

    #[test]
    fn setters() {
        #[ters]
        struct Foo {
            #[allow(unused)]
            a: i32,
            #[get(deref)]
            #[set]
            b: bool,
        }

        let mut foo = Foo { a: 42, b: true };
        assert_eq!(foo.b(), true);
        foo.set_b(false);

        assert_eq!(foo.b(), false);
    }

    #[test]
    fn both() {
        #[ters]
        struct Foo {
            #[get]
            a: i32,
            #[get]
            #[set]
            b: bool,
        }

        let mut foo = Foo { a: 42, b: true };
        assert_eq!(foo.a(), &42);
        assert_eq!(foo.b(), &true);
        foo.set_b(false);

        assert_eq!(foo.b(), &false);
    }
}
