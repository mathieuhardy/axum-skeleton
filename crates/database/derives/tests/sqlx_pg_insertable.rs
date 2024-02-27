use sqlx::{Postgres, QueryBuilder};

use database::traits::sqlx::postgres::crud::SqlxPgInsertable;
use database_derives::SqlxPgInsertable;

#[derive(SqlxPgInsertable)]
struct Foo {
    key_1: Option<String>,
    key_2: Option<i32>,
    key_3: Option<Option<bool>>,
}

#[test]
fn columns() {
    let entry = Foo {
        key_1: Some("foo".to_string()),
        key_2: Some(13),
        key_3: Some(None),
    };

    assert_eq!(
        entry.columns(),
        vec![
            "key_1".to_string(),
            "key_2".to_string(),
            "key_3".to_string()
        ]
    );

    let entry = Foo {
        key_1: Some("foo".to_string()),
        key_2: Some(13),
        key_3: None,
    };

    assert_eq!(
        entry.columns(),
        vec!["key_1".to_string(), "key_2".to_string()]
    );
}

mod bind_insert_values {
    use super::*;

    #[test]
    fn nominal() {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

        let entry = Foo {
            key_1: Some("foo".to_string()),
            key_2: Some(13),
            key_3: Some(None),
        };

        entry.bind_insert_values(&mut query_builder);

        assert_eq!(query_builder.sql(), "($1, $2, $3)");
    }

    #[test]
    fn with_none() {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

        let entry = Foo {
            key_1: Some("foo".to_string()),
            key_2: Some(13),
            key_3: None,
        };

        entry.bind_insert_values(&mut query_builder);

        assert_eq!(query_builder.sql(), "($1, $2)");
    }
}

mod bind_update_values {
    use super::*;

    #[test]
    fn nominal() {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

        let entry = Foo {
            key_1: Some("foo".to_string()),
            key_2: Some(13),
            key_3: Some(None),
        };

        entry.bind_update_values(&mut query_builder, None);

        assert_eq!(query_builder.sql(), "key_1 = $1, key_2 = $2, key_3 = $3");
    }

    #[test]
    fn with_none() {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

        let entry = Foo {
            key_1: Some("foo".to_string()),
            key_2: Some(13),
            key_3: None,
        };

        entry.bind_update_values(&mut query_builder, None);

        assert_eq!(query_builder.sql(), "key_1 = $1, key_2 = $2");
    }

    mod prefix {
        use super::*;

        #[test]
        fn nominal() {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

            let entry = Foo {
                key_1: Some("foo".to_string()),
                key_2: Some(13),
                key_3: Some(None),
            };

            entry.bind_update_values(&mut query_builder, Some("EXCLUDED"));

            assert_eq!(
                query_builder.sql(),
                "key_1 = EXCLUDED.$1, key_2 = EXCLUDED.$2, key_3 = EXCLUDED.$3"
            );
        }

        #[test]
        fn with_none() {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

            let entry = Foo {
                key_1: Some("foo".to_string()),
                key_2: Some(13),
                key_3: None,
            };

            entry.bind_update_values(&mut query_builder, Some("EXCLUDED"));

            assert_eq!(
                query_builder.sql(),
                "key_1 = EXCLUDED.$1, key_2 = EXCLUDED.$2"
            );
        }
    }
}

mod bind_unnest_values {
    use super::*;

    #[test]
    fn nominal() {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

        let foos = [
            Foo {
                key_1: Some("foo_1".to_string()),
                key_2: Some(1),
                key_3: None,
            },
            Foo {
                key_1: Some("foo_2".to_string()),
                key_2: Some(2),
                key_3: None,
            },
        ];

        Foo::bind_unnest_values(&mut query_builder, &foos, false);

        assert_eq!(query_builder.sql(), "UNNEST($1), UNNEST($2)");
    }

    #[test]
    fn as_suffix() {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("");

        let foos = [
            Foo {
                key_1: Some("foo_1".to_string()),
                key_2: Some(1),
                key_3: None,
            },
            Foo {
                key_1: Some("foo_2".to_string()),
                key_2: Some(2),
                key_3: None,
            },
        ];

        Foo::bind_unnest_values(&mut query_builder, &foos, true);

        assert_eq!(
            query_builder.sql(),
            "UNNEST($1) AS key_1, UNNEST($2) AS key_2"
        );
    }
}
