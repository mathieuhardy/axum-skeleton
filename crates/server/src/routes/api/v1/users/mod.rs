//! This file contains all routes dedicated to the users management.

use database::models::users::*;
use database::traits::sqlx::postgres::crud::*;

use crate::prelude::*;
use crate::state::AppState;

/// Builds a router for Kubernetes.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<AppState> {
    Router::new()
        // GET
        .route("/", get(get_filtered))
        .route("/me", get(get_me))
        .route("/:id", get(get_by_id))
        // PATCH
        .route("/:id", patch(patch_user))
        // POST
        .route("/", post(post_user))
        // PUT
        .route("/", put(put_user))
}

/// Handler used to get information about the currently logged user.
#[axum::debug_handler]
#[instrument]
async fn get_me(State(state): State<AppState>) -> Res<Json<User>> {
    let user = User::find_by_filters(
        &Filters {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            ..Filters::default()
        },
        &state.db,
    )
    .await?
    .try_into()?;

    Ok(Json(user))
}

/// Handler used to get a list of users that match some filters.
#[axum::debug_handler]
#[instrument]
async fn get_filtered(
    Query(filters): Query<Filters>,
    State(state): State<AppState>,
) -> Res<Json<Vec<User>>> {
    let users = User::find_by_filters(&filters, &state.db).await?;

    Ok(Json(users))
}

/// Handler used to get a specify user by providing its ID.
#[axum::debug_handler]
#[instrument]
async fn get_by_id(Path(id): Path<Uuid>, State(state): State<AppState>) -> Res<Json<User>> {
    let user = User::get(&id, &state.db).await?;

    Ok(Json(user))
}

/// Handler used to create a new user.
#[axum::debug_handler]
#[instrument]
async fn post_user(
    State(state): State<AppState>,
    FormOrJson(user): FormOrJson<UserRequest>,
) -> Res<Json<User>> {
    user.validate()?;

    let user = User::insert(&user.into(), &state.db).await?;

    Ok(Json(user))
}

/// Handler used to update an existing user by providing its ID.
#[axum::debug_handler]
#[instrument]
async fn patch_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    FormOrJson(user): FormOrJson<UserRequest>,
) -> Res<Json<User>> {
    user.validate()?;

    let user = User::update_by_id(&id, &user.into(), &state.db).await?;

    Ok(Json(user))
}

/// Handler used to upsert a user.
#[axum::debug_handler]
#[instrument]
async fn put_user(
    State(state): State<AppState>,
    FormOrJson(user): FormOrJson<UserRequest>,
) -> Res<Json<User>> {
    user.validate()?;

    let user = if let Some(id) = user.id {
        User::update_by_id(&id, &user.into(), &state.db).await
    } else {
        User::insert(&user.into(), &state.db).await
    }?;

    Ok(Json(user))
}
