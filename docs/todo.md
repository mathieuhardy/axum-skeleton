# ✅ TODO

- global: avoid code in files mod.rs
- user: use transaction when needed

- auth: add endpoints to manage the list of sessions (show, invalidate, etc)

- passwordless:
    - user login with email only
    - backend generate a one-time token
    - backend send by email a link (frontend url + token)
    - frontend call login endpoint with the token and the email
    - backend check the token and email
    - backend creates a user account
    - backend creates a session

- TOTP:
    - user login with email only
    - backend generate a TOTP
    - backend send by email a link (frontend url + totp)
    - frontend call login endpoint with the totp and the email
    - backend check the totp and email
    - backend creates a user account
    - backend creates a session

- OAuth:
    - add endpoint like this: GET /oauth/callback?code=abc123&state=random_nonce
    - bakend exchange the code for an access token

    ```
    POST https://oauth2.googleapis.com/token
    {
      code: "abc123",
      client_id: YOUR_CLIENT_ID,
      client_secret: YOUR_SECRET,
      redirect_uri: "https://yourapp.com/oauth/callback",
      grant_type: "authorization_code"
    }
    ```
    
    - backend get user info from the provider

    ```
    GET https://www.googleapis.com/oauth2/v3/userinfo
    Authorization: Bearer ACCESS_TOKEN
    ```

    - backend matches or create the user account
    - backend creates a session



- user: split user and admin endpoints
- user: add routes for self register: /api/users/profile
- user: make password optional
- user: add route for password recovery
    - generate a one-time token (in a dedicated table)
    - set expiration to 15 minutes
    - send the token with frontend url as a link by email
    - add endpoint that updates password receiving the new password and the token
    - validation of the token
    - update the password

- global: use hexa folder in more crates
- global: add missing tracing events for all errors
- hooks: fix issue

- Redis
- OpenApi
- Job queue
- SSE
- Other methods of authentication (OTP, JWT, etc.)
- Rate limiting for authentication (in an Axum middleware)

## Devops

- Kubernetes, terraform, helm

## Links

- [Template](https://github.com/janos-r/axum-template)

### Axum

- [Axum Prometheus Metrics](https://github.com/tokio-rs/axum/tree/main/examples/prometheus-metrics)

### Various

- [Crates](https://gist.github.com/vi/6620975b737a1caecf607e88cf6b7fea)
- [Access](https://github.com/casbin-rs/axum-casbin)
- [OpenAPI](https://docs.rs/aide/latest/aide/axum/index.html)
- [OpenApi](https://docs.rs/okapi-operation/latest/okapi_operation/#example-using-axum-but-without-axum_integration-feature)
- [Prometheus](https://docs.rs/axum-prometheus/latest/axum_prometheus/)
- [Job queue](https://cetra3.github.io/blog/implementing-a-jobq)
- [Workflow](https://github.com/bahdotsh/wrkflw)
- [Memory allocation](https://leapcell.medium.com/optimizing-rust-performance-with-jemalloc-c18057532194)

### Websockets

- [1](https://crates.io/crates/axum-typed-websockets)
- [2](https://blog.devgenius.io/beyond-the-threads-websockets-in-rust-for-seamless-communication-e40d10e8a0e3)
- [3](https://blog.logrocket.com/build-websocket-server-with-rust/#what-websocket)

### Versioning

- [Best pratice](https://www.reddit.com/r/rust/comments/xnnnzq/whats_the_best_practice_for_shipping_multiple)
- [Versio](https://crates.io/crates/versio)
- [Cargo-edit](https://crates.io/crates/cargo-edit)
- [Cargo-workspaces](https://crates.io/crates/cargo-workspaces)

### HTTP codes

- 202 (Accepted): for jobs
- 208 (Already reported): post the same file

- 301 (Moved permanently): for GET, HEAD
- 308 (Permanent redirect): for POST

- 401 (Unauthorized): for user auth
- 403 (Forbidden): for accesses
- 423 (Locked): for anything that can be locked
- 498 (Token expired): for sessions ?

### Hashing

- [Password storage](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
