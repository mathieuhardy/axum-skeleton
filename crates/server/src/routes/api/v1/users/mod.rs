//! This file contains all routes dedicated to the users management.

use actions::users::{create_user, set_user_password, update_user};
use database::models::users::*;
use database::traits::sqlx::postgres::crud::*;

use crate::prelude::*;

/// Builds a router for Kubernetes.
///
/// # Returns
/// An Axum router.
pub fn build() -> Router<AppState> {
    Router::new()
        // DELETE
        .route("/:id", delete(delete::by_id))
        // GET
        .route("/", get(get::filtered))
        .route("/me", get(get::me))
        .route("/:id", get(get::by_id))
        // PATCH
        .route("/:id", patch(patch::user))
        .route("/:id/password", patch(patch::user_password))
        // POST
        .route("/", post(post::user))
        // PUT
        .route("/", put(put::user))
}

/// List of all DELETE routes.
mod delete {
    use super::*;

    /// Handler used to delete a user giving its ID.
    #[axum::debug_handler]
    #[instrument]
    pub async fn by_id(Path(id): Path<Uuid>, State(state): State<AppState>) -> Res<StatusCode> {
        User::delete_by_id(&id, &state.db).await?;

        Ok(StatusCode::NO_CONTENT)
    }
}

/// List of all GET routes.
mod get {
    use super::*;

    /// Handler used to get information about the currently logged user.
    #[axum::debug_handler]
    #[instrument]
    pub async fn me(
        auth_user: AuthenticationUser,
        State(state): State<AppState>,
    ) -> Res<Json<User>> {
        Ok(Json(auth_user.0))
    }

    /// Handler used to get a list of users that match some filters.
    #[axum::debug_handler]
    #[instrument]
    pub async fn filtered(
        Query(filters): Query<Filters>,
        State(state): State<AppState>,
    ) -> Res<Json<Vec<User>>> {
        let users = User::find_by_filters(&filters, &state.db).await?;

        Ok(Json(users))
    }

    /// Handler used to get a specify user by providing its ID.
    #[axum::debug_handler]
    #[instrument]
    pub async fn by_id(Path(id): Path<Uuid>, State(state): State<AppState>) -> Res<Json<User>> {
        let user = User::get(&id, &state.db).await?;

        Ok(Json(user))
    }
}

/// List of all POST routes.
mod post {
    use super::*;

    /// Handler used to create a new user.
    #[axum::debug_handler]
    #[instrument]
    pub async fn user(
        State(state): State<AppState>,
        FormOrJson(request): FormOrJson<UserRequest>,
    ) -> Res<(StatusCode, Json<User>)> {
        request.validate()?;

        let user = create_user(&request, &state.db).await?;

        Ok((StatusCode::CREATED, Json(user)))
    }
}

/// List of all PATCH routes.
mod patch {
    use super::*;

    /// Handler used to update an existing user by providing its ID.
    #[axum::debug_handler]
    #[instrument]
    pub async fn user(
        Path(id): Path<Uuid>,
        State(state): State<AppState>,
        FormOrJson(request): FormOrJson<UserRequest>,
    ) -> Res<Json<User>> {
        request.validate()?;

        let user = update_user(&id, &request, &state.db).await?;

        Ok(Json(user))
    }

    /// Handler used to update an existing user's password by providing its ID.
    #[axum::debug_handler]
    #[instrument]
    pub async fn user_password(
        Path(id): Path<Uuid>,
        State(state): State<AppState>,
        FormOrJson(request): FormOrJson<PasswordUpdateRequest>,
    ) -> Res<StatusCode> {
        request.validate()?;

        set_user_password(&id, &request, &state.db).await?;

        Ok(StatusCode::OK)
    }
}

/// List of all PUT routes.
mod put {
    use super::*;

    /// Handler used to upsert a user.
    #[axum::debug_handler]
    #[instrument]
    pub async fn user(
        State(state): State<AppState>,
        FormOrJson(request): FormOrJson<UserRequest>,
    ) -> Res<(StatusCode, Json<User>)> {
        request.validate()?;

        let (rc, user) = if let Some(id) = request.id {
            let user = update_user(&id, &request, &state.db).await?;
            (StatusCode::OK, user)
        } else {
            let user = create_user(&request, &state.db).await?;
            (StatusCode::CREATED, user)
        };

        Ok((rc, Json(user)))
    }
}
