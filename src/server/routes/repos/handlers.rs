use super::models::{Repository, ListQuery, CreateRepository, UpdateRepository};
use super::{db::Db, error::Error};
use uuid::Uuid;
use validator::Validate;
use warp::{Reply, reply};

// SQL queries as constants
const SQL_LIST_REPOSITORIES: &str = r#"
    WITH filtered_repos AS (
        SELECT *
        FROM repositories r
        WHERE 
            (CASE WHEN $1::text IS NOT NULL 
                THEN (r.name ILIKE '%' || $1 || '%' OR 
                      COALESCE(r.description, '') ILIKE '%' || $1 || '%')
                ELSE true END)
            AND (CASE WHEN $2::text IS NOT NULL 
                    THEN CASE $2
                        WHEN 'public' THEN NOT r.is_private
                        WHEN 'private' THEN r.is_private
                        ELSE true
                    END
                ELSE true END)
            AND (CASE WHEN $3::text IS NOT NULL 
                    THEN r.language = $3
                ELSE true END)
    )
    SELECT 
        id, name, description, owner_id, is_private,
        created_at, updated_at, language,
        stars_count, forks_count, topics
    FROM filtered_repos
    ORDER BY
        CASE WHEN $4::text = 'stars' THEN stars_count END DESC,
        CASE WHEN $4::text = 'forks' THEN forks_count END DESC,
        CASE WHEN $4::text = 'updated' THEN updated_at END DESC,
        CASE WHEN $4::text IS NULL THEN created_at END DESC
    LIMIT $5
    OFFSET ($6 - 1) * $5
"#;

const SQL_CREATE_REPOSITORY: &str = r#"
    INSERT INTO repositories (
        id, name, description, owner_id, is_private,
        created_at, updated_at, language,
        stars_count, forks_count, topics
    ) VALUES (
        gen_random_uuid(), $1, $2, $3, $4,
        CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $5,
        0, 0, $6
    )
    RETURNING 
        id, name, description, owner_id, is_private,
        created_at, updated_at, language,
        stars_count, forks_count, topics
"#;

const SQL_GET_REPOSITORY: &str = r#"
    SELECT 
        id, name, description, owner_id, is_private,
        created_at, updated_at, language,
        stars_count, forks_count, topics
    FROM repositories
    WHERE id = $1
"#;

const SQL_UPDATE_REPOSITORY: &str = r#"
    UPDATE repositories
    SET
        description = COALESCE($2, description),
        is_private = COALESCE($3, is_private),
        language = COALESCE($4, language),
        topics = COALESCE($5, topics),
        updated_at = CURRENT_TIMESTAMP
    WHERE id = $1
    RETURNING 
        id, name, description, owner_id, is_private,
        created_at, updated_at, language,
        stars_count, forks_count, topics
"#;

const SQL_DELETE_REPOSITORY: &str = r#"
    DELETE FROM repositories
    WHERE id = $1
"#;

// Database error handling
fn handle_db_error(error: sqlx::Error) -> Error {
    match error {
        sqlx::Error::Database(e) => {
            if let Some(code) = e.code() {
                match code.as_ref() {
                    "23505" => Error::conflict("A repository with this name already exists"),
                    "23503" => Error::not_found(),
                    _ => Error::internal(e),
                }
            } else {
                Error::internal(e)
            }
        }
        sqlx::Error::RowNotFound => Error::not_found(),
        e => Error::internal(e),
    }
}

// Handler implementations
pub async fn list(
    query: ListQuery,
    db: Db,
) -> Result<impl Reply, warp::Rejection> {
    // Validate query parameters
    query.validate().map_err(Error::validation)?;

    // Default values
    let per_page = query.per_page.unwrap_or(30).min(100);
    let page = query.page.unwrap_or(1).max(1);

    let repos = sqlx::query_as!(
        Repository,
        SQL_LIST_REPOSITORIES,
        query.q.as_deref(),
        query.repo_type.as_deref(),
        query.language.as_deref(),
        query.sort.as_deref(),
        per_page,
        page
    )
    .fetch_all(&*db.pool)
    .await
    .map_err(handle_db_error)?;

    Ok(reply::json(&repos))
}

pub async fn create(
    new_repo: CreateRepository,
    owner_id: Uuid,  // From auth middleware
    db: Db,
) -> Result<impl Reply, warp::Rejection> {
    // Validate input
    new_repo.validate().map_err(Error::validation)?;

    let repo = sqlx::query_as!(
        Repository,
        SQL_CREATE_REPOSITORY,
        new_repo.name,
        new_repo.description,
        owner_id,
        new_repo.is_private,
        new_repo.language,
        new_repo.topics as Vec<String>
    )
    .fetch_one(&*db.pool)
    .await
    .map_err(handle_db_error)?;

    Ok(reply::with_status(
        reply::json(&repo),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn get(
    id: Uuid,
    db: Db,
) -> Result<impl Reply, warp::Rejection> {
    let repo = sqlx::query_as!(
        Repository,
        SQL_GET_REPOSITORY,
        id
    )
    .fetch_optional(&*db.pool)
    .await
    .map_err(handle_db_error)?
    .ok_or(Error::not_found())?;

    Ok(reply::json(&repo))
}

pub async fn update(
    id: Uuid,
    update: UpdateRepository,
    db: Db,
) -> Result<impl Reply, warp::Rejection> {
    // Validate input
    update.validate().map_err(Error::validation)?;

    let repo = sqlx::query_as!(
        Repository,
        SQL_UPDATE_REPOSITORY,
        id,
        update.description,
        update.is_private,
        update.language,
        update.topics.as_ref().map(|t| t.as_slice())
    )
    .fetch_optional(&*db.pool)
    .await
    .map_err(handle_db_error)?
    .ok_or(Error::not_found())?;

    Ok(reply::json(&repo))
}

pub async fn delete(
    id: Uuid,
    db: Db,
) -> Result<impl Reply, warp::Rejection> {
    sqlx::query!(
        SQL_DELETE_REPOSITORY,
        id
    )
    .execute(&*db.pool)
    .await
    .map_err(handle_db_error)?;

    Ok(reply::with_status(
        reply::json(&()),
        warp::http::StatusCode::NO_CONTENT,
    ))
}