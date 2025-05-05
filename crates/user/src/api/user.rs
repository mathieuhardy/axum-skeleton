//! HTTP endpoints for user management (mostly by an admin user).

use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post, put};
use axum::{Json, Router};
use validator::Validate;

use auth::{Auth, SQLxAuthStore};
use common_core::{AppState, UseCase};
use common_web::extractor::FormOrJson;
use database::extractor::DbPool;

use crate::application::*;
use crate::domain::user::{
    CreateUserRequest, PasswordUpdateRequest, UpdateUserRequest, UpsertUserRequest, User,
    UserFilters,
};
use crate::infrastructure::user::SQLxUserStore;
use crate::prelude::*;

/// Builds an Axum router.
///
/// # Returns
/// An Axum router.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:user_id", delete(delete_user_by_id))
        .route("/current", get(get_current_user))
        .route("/:user_id", get(get_user_by_id))
        .route("/", get(get_users_by_filters))
        .route("/:user_id", patch(update_user))
        .route("/:user_id/password", patch(set_user_password))
        .route("/", post(create_user))
        .route("/", put(upsert_user))
}

/// Handler used to delete a user giving its ID.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn delete_user_by_id(
    Path(user_id): Path<Uuid>,
    DbPool(db): DbPool,
    auth: Auth<SQLxAuthStore>,
) -> ApiResult<impl IntoResponse> {
    if !auth.try_user()?.is_admin() {
        return Err(Error::Forbidden);
    }

    let stores = DeleteUserByIdStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    DeleteUserById::new(stores).handle(user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Handler used to get information about the currently logged user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn get_current_user(
    auth: Auth<SQLxAuthStore>,
    DbPool(db): DbPool,
) -> ApiResult<Json<User>> {
    let stores = GetUserByIdStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    let user = GetUserById::new(stores).handle(auth.try_user()?.id).await?;

    Ok(Json(user))
}

/// Handler used to get a specify user by providing its ID.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
    auth: Auth<SQLxAuthStore>,
    DbPool(db): DbPool,
) -> ApiResult<impl IntoResponse> {
    if !auth.try_user()?.is_admin() {
        return Err(Error::Forbidden);
    }

    let stores = GetUserByIdStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    let user = GetUserById::new(stores).handle(user_id).await?;

    Ok(Json(user))
}

/// Handler used to get a list of users that match some filters.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn get_users_by_filters(
    auth: Auth<SQLxAuthStore>,
    Query(filters): Query<UserFilters>,
    DbPool(db): DbPool,
) -> ApiResult<impl IntoResponse> {
    if !auth.try_user()?.is_admin() {
        return Err(Error::Forbidden);
    }

    let stores = GetUsersByFiltersStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    let users = GetUsersByFilters::new(stores).handle(filters).await?;

    Ok(Json(users))
}

/// Handler used to create a new user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn create_user(
    auth: Auth<SQLxAuthStore>,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<CreateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    if !auth.try_user()?.is_admin() {
        return Err(Error::Forbidden);
    }

    request.validate()?;

    let stores = CreateUserStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    let user = CreateUser::new(stores).handle(request).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Handler used to upsert a user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn upsert_user(
    auth: Auth<SQLxAuthStore>,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<UpsertUserRequest>,
) -> ApiResult<impl IntoResponse> {
    let user = auth.try_user()?;

    let rc = match request.user_id {
        Some(user_id) => {
            // Update of existing user
            if !user.is_admin() && !user.is(&user_id) {
                return Err(Error::Forbidden);
            }

            request.validate()?;

            StatusCode::OK
        }

        None => {
            // Creation
            if !user.is_admin() {
                return Err(Error::Forbidden);
            }

            request.validate()?;

            StatusCode::CREATED
        }
    };

    let stores = UpsertUserStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    let user = UpsertUser::new(stores).handle(request).await?;

    Ok((rc, Json(user)))
}

/// Handler used to update a user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn update_user(
    Path(user_id): Path<Uuid>,
    auth: Auth<SQLxAuthStore>,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<UpdateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    let user = auth.try_user()?;

    if !user.is_admin() && !user.is(&user_id) {
        return Err(Error::Forbidden);
    }

    request.validate()?;

    let stores = UpdateUserStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    let user = UpdateUser::new(stores).handle((user_id, request)).await?;

    Ok(Json(user))
}

/// Handler used to update an existing user's password by providing its ID.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn set_user_password(
    Path(user_id): Path<Uuid>,
    auth: Auth<SQLxAuthStore>,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<PasswordUpdateRequest>,
) -> ApiResult<impl IntoResponse> {
    if !auth.try_user()?.is(&user_id) {
        return Err(Error::Forbidden);
    }

    request.validate()?;

    let stores = SetUserPasswordStores {
        user: Arc::new(SQLxUserStore::new(db)),
    };

    SetUserPassword::new(stores)
        .handle((user_id, request))
        .await?;

    Ok(StatusCode::OK)
}
