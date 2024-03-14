use sqlx::{sqlite::SqlitePool, Pool, Sqlite};

mod query;
use query as query_select;
pub mod model;

type ActualDatabase = Pool<Sqlite>;

pub async fn get_users() -> anyhow::Result<Vec<model::DbUser>> {
    let pool = SqlitePool::connect("banco.sqlite3").await?;
    let recs = sqlx::query_as::<_, model::DbUser>(query_select::SELECT_USER_INFOMATION)
        .fetch_all(&pool)
        .await?;

    Ok(recs)
}

pub async fn update_user(user: model::DbUser) -> anyhow::Result<()> {
    let poll = SqlitePool::connect("banco.sqlite3").await?;
    let _ = sqlx::query(query_select::UPDADE_USER_INFORMATION)
        .bind(user.name)
        .bind(user.email)
        .bind(user.login)
        .execute(&poll)
        .await?
        .rows_affected();
    Ok(())
}

pub async fn get_computers(db: &ActualDatabase) -> anyhow::Result<Vec<model::DbComputer>> {
    let recs = sqlx::query_as::<_, model::DbComputer>(
        query_select::SELECT_COMPUTER_INFORMATION_WITH_LAST_USER,
    )
    .fetch_all(db)
    .await?;
    Ok(recs)
}
pub async fn update_user_equipament(
    actual_user: String,
    future_user: String,
    equipament: String,
) -> anyhow::Result<()> {
    let poll = SqlitePool::connect("banco.sqlite3").await?;
    let today = chrono::Local::now().date_naive().to_string();
    dbg!(&equipament, &actual_user, &future_user, &today);
    let _ = sqlx::query(query_select::UPDATE_LAST_USER_COMPUTER)
        .bind(&today)
        .bind(&actual_user)
        .bind(&equipament)
        .execute(&poll)
        .await
        .map_err(|e| dbg!(e));
    // dbg!(recs);
    let recs = sqlx::query(query_select::INSERT_NEW_USER_COMPUTER)
        .bind(&equipament)
        .bind(&future_user)
        .bind(&today)
        .execute(&poll)
        .await?;
    dbg!(recs);

    // .rows_affected();
    Ok(())
}
