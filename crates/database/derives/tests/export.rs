use database_derives::{export, Export};

#[test]
fn no_name() {
    #[allow(dead_code)]
    #[derive(Export)]
    struct Foo {}
}

pub mod single {
    use super::*;

    #[test]
    fn nominal() {
        #[allow(dead_code)]
        #[derive(Export)]
        #[export(Bar)]
        struct Foo {}

        let _ = FooBar {};
    }

    #[test]
    fn without_fields() {
        #[allow(dead_code)]
        #[derive(Export)]
        #[export(Bar)]
        struct Foo {
            private: String,
            pub public: bool,
            pub value: Option<i32>,
        }

        let _ = FooBar {};
    }

    #[test]
    fn with_fields() {
        #[allow(dead_code)]
        #[derive(Export)]
        #[export(Bar)]
        struct Foo {
            #[allow(dead_code)]
            #[is_in(Bar)]
            private: String,
            #[is_in(Bar)]
            pub public: bool,
            #[is_in(Bar)]
            pub value: Option<i32>,
        }

        let _ = FooBar {
            private: "".to_string(),
            public: true,
            value: Some(42),
        };
    }

    #[test]
    fn with_optional_field() {
        #[allow(dead_code)]
        #[derive(Export)]
        #[export(Bar)]
        struct Foo {
            #[optional_in(Bar)]
            pub value: Option<i32>,
        }

        let _ = FooBar {
            value: Some(Some(42)),
        };
    }
}

pub mod multiple {
    use super::*;

    #[test]
    fn nominal() {
        #[allow(dead_code)]
        #[derive(Export)]
        #[export(Foo, Bar)]
        struct Foo {}

        let _ = FooFoo {};
        let _ = FooBar {};
    }
}

pub mod derives {
    use super::*;

    #[test]
    fn single() {
        #[allow(dead_code)]
        #[derive(Export)]
        #[export(Bar)]
        #[export(derives(Bar(Clone)))]
        struct Foo {}

        let bar = FooBar {};
        let _ = bar.clone();
    }

    #[test]
    fn multiple() {
        #[allow(dead_code)]
        #[derive(Export)]
        #[export(Bar)]
        #[export(derives(Bar(Debug, Clone)))]
        struct Foo {}

        let bar = FooBar {};
        let _ = bar.clone();
        println!("{:?}", bar);
    }
}
