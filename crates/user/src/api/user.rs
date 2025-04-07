use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post, put};
use axum::{Json, Router};
use validator::Validate;

use auth::AuthUser;
use common_core::{AppState, UseCase};
use common_web::extractor::FormOrJson;
use database::extractor::DbPool;

use crate::application::*;
use crate::domain::user::{
    CreateUserRequest, PasswordUpdateRequest, UpdateUserRequest, UpsertUserRequest, User,
    UserFilters,
};
use crate::infrastructure::user::SQLxUserRepository;
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
    auth_user: AuthUser,
) -> ApiResult<impl IntoResponse> {
    if !auth_user.is_admin() {
        return Err(Error::Forbidden);
    }

    let repos = DeleteUserByIdRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    DeleteUserById::new(repos).handle(user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Handler used to get information about the currently logged user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn get_current_user(auth_user: AuthUser, DbPool(db): DbPool) -> ApiResult<Json<User>> {
    if !auth_user.is_admin() {
        return Err(Error::Forbidden);
    }

    let repos = GetUserByIdRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    let user_id = auth_user.id;

    let user = GetUserById::new(repos).handle(user_id).await?;

    Ok(Json(user))
}

/// Handler used to get a specify user by providing its ID.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
    DbPool(db): DbPool,
) -> ApiResult<impl IntoResponse> {
    if !auth_user.is_admin() {
        return Err(Error::Forbidden);
    }

    let repos = GetUserByIdRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    let user = GetUserById::new(repos).handle(user_id).await?;

    Ok(Json(user))
}

/// Handler used to get a list of users that match some filters.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn get_users_by_filters(
    auth_user: AuthUser,
    Query(filters): Query<UserFilters>,
    DbPool(db): DbPool,
) -> ApiResult<impl IntoResponse> {
    if !auth_user.is_admin() {
        return Err(Error::Forbidden);
    }

    let repos = GetUsersByFiltersRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    let users = GetUsersByFilters::new(repos).handle(filters).await?;

    Ok(Json(users))
}

/// Handler used to create a new user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn create_user(
    auth_user: AuthUser,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<CreateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    if !auth_user.is_admin() {
        return Err(Error::Forbidden);
    }

    request.validate()?;

    let repos = CreateUserRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    let user = CreateUser::new(repos).handle(request).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Handler used to upsert a user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn upsert_user(
    auth_user: AuthUser,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<UpsertUserRequest>,
) -> ApiResult<impl IntoResponse> {
    let rc = match request.user_id {
        Some(user_id) => {
            // Update of existing user
            if !auth_user.is_admin() && !auth_user.is(&user_id) {
                return Err(Error::Forbidden);
            }

            request.validate()?;

            StatusCode::OK
        }

        None => {
            // Creation
            if !auth_user.is_admin() {
                return Err(Error::Forbidden);
            }

            request.validate()?;

            StatusCode::CREATED
        }
    };

    let repos = UpsertUserRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    let user = UpsertUser::new(repos).handle(request).await?;

    Ok((rc, Json(user)))
}

/// Handler used to update a user.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn update_user(
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<UpdateUserRequest>,
) -> ApiResult<impl IntoResponse> {
    if !auth_user.is_admin() && !auth_user.is(&user_id) {
        return Err(Error::Forbidden);
    }

    request.validate()?;

    let repos = UpdateUserRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    let user = UpdateUser::new(repos).handle((user_id, request)).await?;

    Ok(Json(user))
}

/// Handler used to update an existing user's password by providing its ID.
#[instrument]
#[axum::debug_handler(state = AppState)]
pub async fn set_user_password(
    Path(user_id): Path<Uuid>,
    auth_user: AuthUser,
    DbPool(db): DbPool,
    FormOrJson(request): FormOrJson<PasswordUpdateRequest>,
) -> ApiResult<impl IntoResponse> {
    if !auth_user.is(&user_id) {
        return Err(Error::Forbidden);
    }

    request.validate()?;

    let repos = SetUserPasswordRepos {
        user: Arc::new(SQLxUserRepository::new(db)),
    };

    SetUserPassword::new(repos)
        .handle((user_id, request))
        .await?;

    Ok(StatusCode::OK)
}
