use std::{path::Path, sync::Arc};

use sqlx::{sqlite::SqlitePool, Pool, Sqlite};

mod query;
use query as query_select;
pub mod model;

fn data_base_directory() -> Arc<String> {
    use std::env;
    let dir = match env::var("SIMPLE_INVENTARY_DATABASE_PATH") {
        Ok(path) => Arc::new(path),
        _ => {
            let path = Path::new("./").join("database.sqlite3");
            if path.exists() {
                return Arc::new(path.to_string_lossy().to_string());
            }
            Arc::new(path.to_string_lossy().to_string()) // TODO: use XDG path
        }
    };
    dir //TODO: check if database already exist, create if not
}

pub async fn get_brands() -> anyhow::Result<Vec<model::DbBrand>> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbBrand>(query_select::SELECT_BRAND)
        .fetch_all(&pool)
        .await?;
    Ok(recs)
}
pub async fn get_cpus() -> anyhow::Result<Vec<model::DbCPU>> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbCPU>(query_select::SELECT_CPU)
        .fetch_all(&pool)
        .await?;

    Ok(recs)
}

pub async fn delete_cpu(name: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::DELETE_CPU)
        .bind(name)
        .execute(&poll)
        .await;
    Ok(())
}
pub async fn insert_cpu(name: String, brand: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::INSERT_CPU)
        .bind(name.to_uppercase())
        .bind(brand)
        .execute(&poll)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;
    Ok(())
}
pub async fn get_equipament_model() -> anyhow::Result<Vec<model::DbEquipamentModel>> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbEquipamentModel>(query_select::SELECT_EQUIPAMENT_MODEL)
        .fetch_all(&pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;

    Ok(recs)
}

pub async fn delete_equipament_model(name: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::DELETE_EQUIPAMENT_MODEL)
        .bind(name)
        .execute(&poll)
        .await;
    Ok(())
}
pub async fn insert_equipament_model(name: String, brand: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    dbg!(&name, &brand);
    let _ = sqlx::query(query_select::INSERT_EQUIPAMENT_MODEL)
        .bind(name.to_uppercase())
        .bind(brand)
        .execute(&poll)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;
    Ok(())
}
pub async fn get_gpus() -> anyhow::Result<Vec<model::DbGPU>> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbGPU>(query_select::SELECT_GPU)
        .fetch_all(&pool)
        .await?;

    Ok(recs)
}
pub async fn get_users() -> anyhow::Result<Vec<model::DbUser>> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbUser>(query_select::SELECT_USER_INFOMATION)
        .fetch_all(&pool)
        .await;
    Ok(recs?)
}
pub async fn get_specific_user(login: String) -> anyhow::Result<model::DbUser> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbUser>(query_select::SELECT_SPECIFIC_USER_INFOMATION)
        .bind(&login)
        .fetch_one(&pool)
        .await;
    Ok(recs?)
}
pub async fn delete_brand(name: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::DELETE_BRAND)
        .bind(name)
        .execute(&poll)
        .await?;

    Ok(())
}

pub async fn insert_brand(name: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::INSERT_BRAND)
        .bind(name.to_uppercase())
        .execute(&poll)
        .await?;
    Ok(())
}

pub async fn get_department() -> anyhow::Result<Vec<model::DbDepartment>> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbDepartment>(query_select::SELECT_DEPARTMENT)
        .fetch_all(&pool)
        .await?;

    Ok(recs)
}

pub async fn get_department_by_id(id: String) -> anyhow::Result<model::DbDepartment> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbDepartment>(query_select::SELECT_DEPARTMENT_BY_ID)
        .bind(id)
        .fetch_one(&pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;

    Ok(recs)
}
pub async fn get_department_by_name(name: String) -> anyhow::Result<model::DbInteger> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbInteger>(query_select::SELECT_DEPARTMENT_BY_NAME)
        .bind(name)
        .fetch_one(&pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;

    Ok(recs)
}
pub async fn delete_department(name: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::DELETE_DEPARTMENT)
        .bind(name)
        .execute(&poll)
        .await?;

    Ok(())
}

pub async fn insert_department(name: String) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::INSERT_DEPARTMENT)
        .bind(name)
        .execute(&poll)
        .await?;
    Ok(())
}

pub async fn update_user(user: model::DbUser) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    dbg!(&user);
    let _ = sqlx::query(query_select::UPDADE_USER_INFORMATION)
        .bind(user.name)
        .bind(user.email)
        .bind(user.login)
        .execute(&poll)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        })?
        .rows_affected();
    Ok(())
}

pub async fn create_user(user: model::DbUser) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::INSERT_USER_INFORMATION)
        .bind(user.name.to_lowercase())
        .bind(user.department)
        .bind(user.document)
        .bind(user.email)
        .bind(user.login)
        .bind(user.extension)
        .bind(user.phone_number)
        .execute(&poll)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        });
    Ok(())
}

pub async fn create_computer(computer: model::DbComputer) -> anyhow::Result<()> {
    dbg!(&computer);
    let poll = get_sql_pool().await?;
    let _ = sqlx::query(query_select::INSERT_COMPUTER)
        .bind(computer.serialnumber)
        .bind(computer.brand)
        .bind(computer.cpu)
        .bind(computer.storage)
        .bind(computer.memory)
        .bind(computer.model)
        .bind(computer.type_equipament)
        .bind(computer.observation)
        .execute(&poll)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        });
    Ok(())
}
async fn get_sql_pool() -> anyhow::Result<Pool<Sqlite>> {
    Ok(SqlitePool::connect(&data_base_directory())
        .await
        .inspect_err(|e| {
            dbg!(&e);
        })?)
}

pub async fn get_computers() -> anyhow::Result<Vec<model::DbComputer>> {
    let poll = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, model::DbComputer>(
        query_select::SELECT_COMPUTER_INFORMATION_WITH_LAST_USER,
    )
    .fetch_all(&poll)
    .await?;
    Ok(recs)
}
pub async fn update_user_equipament(
    actual_user: String,
    future_user: String,
    equipament: String,
) -> anyhow::Result<()> {
    let poll = get_sql_pool().await?;
    let today = chrono::Local::now().date_naive().to_string();
    let _ = sqlx::query(query_select::UPDATE_LAST_USER_COMPUTER)
        .bind(&today)
        .bind(&actual_user)
        .bind(&equipament)
        .execute(&poll)
        .await?;
    let _ = sqlx::query(query_select::INSERT_NEW_USER_COMPUTER)
        .bind(&equipament)
        .bind(&future_user)
        .bind(&today)
        .execute(&poll)
        .await?;
    Ok(())
}
