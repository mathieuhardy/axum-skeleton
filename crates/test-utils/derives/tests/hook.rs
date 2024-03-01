use test_utils::*;

async fn setup_success() {}

async fn setup_failure() {
    panic!();
}

async fn teardown_success<T>(_: T) {}

async fn teardown_failure<T>(_: T) {
    panic!();
}

mod none {
    use super::*;

    #[hook(_, _)]
    #[tokio::test]
    async fn success() {
        |_| async move {}
    }
}

mod body {
    use super::*;

    #[hook(_, _)]
    #[tokio::test]
    #[should_panic]
    async fn failure() {
        |_| async move {
            panic!();
        }
    }
}

mod setup {
    use super::*;

    #[hook(setup_success, _)]
    #[tokio::test]
    async fn success() {
        |_| async move {}
    }

    #[hook(setup_failure, _)]
    #[tokio::test]
    #[should_panic]
    async fn failure() {
        |_| async move {}
    }
}

mod teardown {
    use super::*;

    #[hook(_, teardown_success)]
    #[tokio::test]
    async fn success() {
        |_| async move {}
    }

    #[hook(_, teardown_failure)]
    #[tokio::test]
    #[should_panic]
    async fn failure() {
        |_| async move {}
    }
}

mod both {
    use super::*;

    #[hook(setup_success, teardown_success)]
    #[tokio::test]
    async fn success() {
        |_| async move {}
    }

    #[hook(setup_failure, teardown_success)]
    #[tokio::test]
    #[should_panic]
    async fn failure_setup() {
        |_| async move {}
    }

    #[hook(setup_success, teardown_failure)]
    #[tokio::test]
    #[should_panic]
    async fn failure_teardown() {
        |_| async move {}
    }

    #[hook(setup_failure, teardown_failure)]
    #[tokio::test]
    #[should_panic]
    async fn failure_both() {
        |_| async move {}
    }
}
