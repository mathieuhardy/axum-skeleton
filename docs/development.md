# 💻 Development

- [🪝 Git Hooks](development/git-hooks.md)
- [🗃  SQLx usage](development/sqlx.md)
- [📄 Logging system](development/logging.md)
- [💯 Testing](development/testing.md)
- [🖊 Coding style](development/coding-style.md)
- [🩺 Sanity](development/sanity.md)

## Database

In the crate `database`, in the folder `scripts/sql` are stored some SQL
scripts. They intend to be used by SQLx as plain text request. These requests
can receive parameters by placing identifiers like this: `$1`, `$2`, etc.

The file `build.rs` is called before running compilation and will take all
scripts in this directory and create a `src/requests.rs` file that contains all
requests stored in Rust constants available for SQLx usage.

## Routes

Every POST/PUT route must allows to receive JSON or form data. This can be done
easily using the type `FormOrJson`:

```rust
async fn post_handler(FormOrJson(data): FormOrJson<Data>) -> Json<Response> {
    // ...
}
```

## Pull requests

The GitHub template is located at `.github/pull_request_template.md` and can be
modified to whatever suits you.
