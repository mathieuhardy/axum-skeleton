# 💻 Development

- [🪝 Git Hooks](development/git-hooks.md)
- [🗃  SQLx usage](development/sqlx.md)

- [🧱 Architecture](development/architecture.md)
- [📄 Logging system](development/logging.md)
- [💯 Testing](development/testing.md)
- [🖊 Coding style](development/coding-style.md)
- [🩺 Sanity](development/sanity.md)

## Database


All base stuff is located in the `database` crate. It contains for example the
migrations and the base functions to initialize the database. It also provides
extractors for Axum endpoints.

There's no more here as the requests will be declared in each hexagonal crate.

## Routes

Every PATCH/POST/PUT route must allows to receive JSON or form data. This can be
done easily using the extractor `FormOrJson`:

```rust
async fn post_handler(FormOrJson(data): FormOrJson<Data>) -> Json<Response> {
    // ...
}
```

## Pull requests

The GitHub template is located at `.github/pull_request_template.md` and can be
modified to whatever suits you.
