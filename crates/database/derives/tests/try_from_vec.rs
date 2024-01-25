use database_derives::TryFromVec;

#[test]
fn nominal() {
    #[derive(Debug)]
    pub enum Error {
        NotFound,
    }

    #[derive(Clone, TryFromVec)]
    struct Foo {
        is_set: bool,
    }

    let list = vec![Foo { is_set: true }, Foo { is_set: false }];
    let single: Foo = list.try_into().unwrap();
    assert!(single.is_set);

    let list = vec![Foo { is_set: false }, Foo { is_set: true }];
    let single: Foo = list.try_into().unwrap();
    assert!(!single.is_set);
}
