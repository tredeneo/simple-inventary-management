use std::{fs, ops::Not, path::Path, sync::Arc};

use sqlx::{
    Connection, SqliteConnection,
    sqlite::{SqliteConnectOptions, SqlitePool},
};

mod query;
use query as query_select;

pub mod model;

async fn data_base_directory() -> Arc<String> {
    use std::env;

    let _ = get_xdg_database_path();
    if let Ok(path) = env::var("SIMPLE_INVENTARY_DATABASE_PATH") {
        return Arc::new(path);
    }

    let local_path = Path::new("./database.sqlite3");
    if local_path.exists() {
        return Arc::new(local_path.to_string_lossy().to_string());
    }
    let xdg_dir = get_xdg_database_path();
    create_database(xdg_dir.clone()).await.ok();
    Arc::new(xdg_dir)
}

fn get_xdg_database_path() -> String {
    use directories::BaseDirs;

    let dir = BaseDirs::new().unwrap();
    let mut dir = dir.config_local_dir().to_path_buf();
    dir.push("simple_inventary");
    if dir.exists().not() {
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("Erro ao criar diretório: {}", e);
        }
    }
    dir.push("database.sqlite3");
    dir.to_string_lossy().to_string()
}
// async fn create_database(location: &Path) -> anyhow::Result<()> {
async fn create_database(location: String) -> anyhow::Result<()> {
    let tmp = SqliteConnectOptions::new()
        // .filename(location.to_string_lossy().to_string())
        .filename(location)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(tmp)
        .await
        .inspect(|ok| {
            dbg!(ok);
        })
        .inspect_err(|e| {
            dbg!(e);
        })?;
    sqlx::query(
        r#"
        CREATE TABLE "CPU" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"brand"	INTEGER NOT NULL,
        	"name"	TEXT NOT NULL UNIQUE,
        	FOREIGN KEY("brand") REFERENCES "brands"("id"),
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        CREATE TABLE "GPU" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"brand"	INTEGER NOT NULL,
        	"name"	TEXT NOT NULL UNIQUE,
        	FOREIGN KEY("brand") REFERENCES "brands"("id"),
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        CREATE TABLE "brands" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"name"	TEXT NOT NULL UNIQUE,
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        CREATE TABLE "departments" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"name"	TEXT NOT NULL UNIQUE,
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        
        CREATE TABLE "equipament_model" (
            "id" INTEGER NOT NULL UNIQUE,
            "name" TEXT NOT NULL UNIQUE,
            "brand" INTEGER NOT NULL,
            "cpu" INTEGER NOT NULL,
            "gpu" INTEGER NOT NULL,
            "smartphone" INTEGER DEFAULT 0,
            FOREIGN KEY ("brand") REFERENCES "brands" ("id"),
            FOREIGN KEY ("gpu") REFERENCES "GPU" ("id"),
            FOREIGN KEY ("cpu") REFERENCES "CPU" ("id"),
            PRIMARY KEY ("id" AUTOINCREMENT)
          );
          
        CREATE TABLE "equipaments" (
        	"serialnumber"	TEXT NOT NULL UNIQUE,
        	"storage"	INTEGER NOT NULL,
        	"memory"	INTEGER NOT NULL,
        	"model"	TEXT NOT NULL,
        	"id"	INTEGER NOT NULL UNIQUE,
        	"observation"	TEXT,
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        CREATE TABLE "has" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"computer_id"	INTEGER NOT NULL,
        	"user_id"	INTEGER NOT NULL,
        	"date_begin"	TEXT NOT NULL,
        	"date_end"	TEXT DEFAULT null,
        	FOREIGN KEY("computer_id") REFERENCES "equipaments"("id"),
        	FOREIGN KEY("user_id") REFERENCES "users"("id"),
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        CREATE TABLE "models" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"name"	TEXT UNIQUE,
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        CREATE TABLE "phone_number" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"number"	TEXT,
        	PRIMARY KEY("id" AUTOINCREMENT)
        );
        
        CREATE TABLE
          "phone_number" (
            "id" INTEGER NOT NULL UNIQUE,
            "user_id" INT NOT NULL UNIQUE,
            "number" VARCHAR(255) NOT NULL,
            PRIMARY KEY ("id" AUTOINCREMENT)
          );   
        CREATE TABLE type (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, category NOT NULL);
        CREATE TABLE "users" (
        	"id"	INTEGER NOT NULL UNIQUE,
        	"name"	TEXT NOT NULL UNIQUE,
        	"department"	TEXT NOT NULL,
        	"document"	TEXT UNIQUE,
        	"email"	TEXT NOT NULL,
        	"login"	TEXT NOT NULL UNIQUE,
        	"extension"	TEXT,
        	"phone_number"	TEXT,
        	"active"	INTEGER NOT NULL DEFAULT 1,
        	PRIMARY KEY("id" AUTOINCREMENT),
        	FOREIGN KEY("department") REFERENCES "departments"("id")
        );
        
    "#,
    )
    .execute(&pool)
    .await?;

    println!("Banco de dados criado (se necessário) e conectado com sucesso!");

    Ok(())
}

async fn get_sql_connection() -> anyhow::Result<SqliteConnection> {
    Ok(SqliteConnection::connect(&data_base_directory().await)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        })?)
}
pub async fn get_equipaments_by_users(
    user_id: String,
) -> anyhow::Result<Vec<model::DbEquipamentHistoric>> {
    let mut pool = get_sql_connection().await?;
    let recs =
        sqlx::query_as::<_, model::DbEquipamentHistoric>(query_select::SELECT_COMPUTERS_BY_USER)
            .bind(user_id)
            .fetch_all(&mut pool)
            .await
            .inspect_err(|e| {
                dbg!(&e);
            })?;
    Ok(recs)
}

pub async fn get_brands() -> anyhow::Result<Vec<model::DbBrand>> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbBrand>(query_select::SELECT_BRAND)
        .fetch_all(&mut pool)
        .await?;
    Ok(recs)
}
pub async fn get_cpus() -> anyhow::Result<Vec<model::DbCPU>> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbCPU>(query_select::SELECT_CPU)
        .fetch_all(&mut pool)
        .await?;

    Ok(recs)
}

pub async fn delete_cpu(name: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::DELETE_CPU)
        .bind(name)
        .execute(&mut pool)
        .await;
    Ok(())
}
pub async fn insert_cpu(name: String, brand: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::INSERT_CPU)
        .bind(name.to_uppercase())
        .bind(brand)
        .execute(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;
    Ok(())
}
pub async fn get_equipament_model() -> anyhow::Result<Vec<model::DbEquipamentModel>> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbEquipamentModel>(query_select::SELECT_EQUIPAMENT_MODEL)
        .fetch_all(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })
        .unwrap_or_default();

    Ok(recs)
}

pub async fn update_equipament_model(equipament: model::DbEquipamentModel) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::UPDADE_EQUIPAMENT_MODEL_INFORMATION)
        .bind(equipament.name.clone())
        .bind(equipament.brand)
        .bind(equipament.cpu)
        .bind(equipament.gpu)
        .bind(equipament.smartphone)
        .bind(equipament.name)
        .execute(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        })?
        .rows_affected();
    Ok(())
}
pub async fn delete_equipament_model(name: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::DELETE_EQUIPAMENT_MODEL)
        .bind(name)
        .execute(&mut pool)
        .await;
    Ok(())
}
pub async fn insert_equipament_model(equipament: model::DbEquipamentModel) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::INSERT_EQUIPAMENT_MODEL)
        .bind(equipament.name.to_uppercase())
        .bind(equipament.brand)
        .bind(equipament.cpu)
        .bind(equipament.gpu)
        .bind(equipament.smartphone)
        .execute(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;
    Ok(())
}
pub async fn get_gpus() -> anyhow::Result<Vec<model::DbGPU>> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbGPU>(query_select::SELECT_GPU)
        .fetch_all(&mut pool)
        .await?;

    Ok(recs)
}

pub async fn delete_gpu(name: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::DELETE_GPU)
        .bind(name)
        .execute(&mut pool)
        .await;
    Ok(())
}
pub async fn insert_gpu(name: String, brand: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::INSERT_GPU)
        .bind(name.to_uppercase())
        .bind(brand)
        .execute(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })?;
    Ok(())
}
pub async fn get_users() -> anyhow::Result<Vec<model::DbUser>> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbUser>(query_select::SELECT_USER_INFOMATION)
        .fetch_all(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        });
    Ok(recs?)
}
pub async fn get_specific_user_by_login(login: String) -> anyhow::Result<model::DbUser> {
    let mut pool = get_sql_connection().await?;
    let recs =
        sqlx::query_as::<_, model::DbUser>(query_select::SELECT_SPECIFIC_USER_INFOMATION_BY_LOGIN)
            .bind(&login)
            .fetch_one(&mut pool)
            .await;
    Ok(recs?)
}

pub async fn get_specific_user_by_name(login: String) -> anyhow::Result<model::DbUser> {
    let mut pool = get_sql_connection().await?;
    let recs =
        sqlx::query_as::<_, model::DbUser>(query_select::SELECT_SPECIFIC_USER_INFOMATION_BY_NAME)
            .bind(&login)
            .fetch_one(&mut pool)
            .await;
    Ok(recs?)
}
pub async fn get_specific_equipament_model(
    name: String,
) -> anyhow::Result<model::DbEquipamentModel> {
    let mut pool = get_sql_connection().await?;

    let recs = sqlx::query_as::<_, model::DbEquipamentModel>(
        query_select::SELECT_SPECIFIC_EQUIPAMENT_MODEL_INFOMATION,
    )
    .bind(name.to_uppercase())
    .fetch_one(&mut pool)
    .await
    .inspect_err(|e| {
        dbg!(e);
    })
    .unwrap_or_default();
    Ok(recs)
}
pub async fn delete_brand(name: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::DELETE_BRAND)
        .bind(name)
        .execute(&mut pool)
        .await?;

    Ok(())
}

pub async fn insert_brand(name: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::INSERT_BRAND)
        .bind(name.to_uppercase())
        .execute(&mut pool)
        .await?;
    Ok(())
}

pub async fn get_department() -> anyhow::Result<Vec<model::DbDepartment>> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbDepartment>(query_select::SELECT_DEPARTMENT)
        .fetch_all(&mut pool)
        .await?;

    Ok(recs)
}

pub async fn get_department_by_id(id: String) -> anyhow::Result<model::DbDepartment> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbDepartment>(query_select::SELECT_DEPARTMENT_BY_ID)
        .bind(id)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })
        .unwrap_or_default();

    Ok(recs)
}
pub async fn get_department_by_name(name: String) -> anyhow::Result<model::DbInteger> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbInteger>(query_select::SELECT_DEPARTMENT_BY_NAME)
        .bind(name)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(e);
        })
        .unwrap_or_default();

    Ok(recs)
}

pub async fn get_brand_by_id(id: String) -> anyhow::Result<model::DbBrand> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbBrand>(query_select::SELECT_BRAND_BY_ID)
        .bind(id)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })
        .unwrap_or_default();

    Ok(recs)
}
pub async fn get_brand_by_name(name: String) -> anyhow::Result<model::DbInteger> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbInteger>(query_select::SELECT_BRAND_BY_NAME)
        .bind(name)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(e);
        })
        .unwrap_or_default();

    Ok(recs)
}

pub async fn get_gpu_by_id(id: String) -> anyhow::Result<model::DbGPU> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbGPU>(query_select::SELECT_GPU_BY_ID)
        .bind(id)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })
        .unwrap_or_default();

    Ok(recs)
}
pub async fn get_gpu_by_name(name: String) -> anyhow::Result<model::DbInteger> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbInteger>(query_select::SELECT_GPU_BY_NAME)
        .bind(name)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(e);
        })
        .unwrap_or_default();

    Ok(recs)
}

pub async fn get_cpu_by_id(id: String) -> anyhow::Result<model::DbCPU> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbCPU>(query_select::SELECT_CPU_BY_ID)
        .bind(id)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|f| {
            dbg!(f);
        })
        .unwrap_or_default();

    Ok(recs)
}
pub async fn get_cpu_by_name(name: String) -> anyhow::Result<model::DbInteger> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbInteger>(query_select::SELECT_CPU_BY_NAME)
        .bind(name)
        .fetch_one(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(e);
        })
        .unwrap_or_default();

    Ok(recs)
}
pub async fn delete_department(name: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::DELETE_DEPARTMENT)
        .bind(name)
        .execute(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        })?;

    Ok(())
}

pub async fn insert_department(name: String) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::INSERT_DEPARTMENT)
        .bind(name)
        .execute(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        })?;
    Ok(())
}

pub async fn update_user(user: model::DbUser) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    dbg!(&user);
    let _ = sqlx::query(query_select::UPDADE_USER_INFORMATION)
        .bind(user.name)
        .bind(user.email)
        .bind(user.login)
        .bind(user.phone_number)
        .bind(user.department)
        .bind(user.extension)
        .bind(user.document)
        .execute(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        })?
        .rows_affected();
    Ok(())
}

pub async fn create_user(user: model::DbUser) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::INSERT_USER_INFORMATION)
        .bind(user.name.to_lowercase())
        .bind(user.department)
        .bind(user.document)
        .bind(user.email)
        .bind(user.login)
        .bind(user.extension)
        .bind(user.phone_number)
        .execute(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        });
    Ok(())
}

pub async fn create_computer(computer: model::DbComputer) -> anyhow::Result<()> {
    dbg!(&computer);
    let mut pool = get_sql_connection().await?;
    let _ = sqlx::query(query_select::INSERT_COMPUTER)
        .bind(computer.serialnumber.to_uppercase())
        .bind(computer.storage)
        .bind(computer.memory)
        .bind(computer.model)
        .bind(computer.observation)
        .execute(&mut pool)
        .await
        .inspect_err(|e| {
            dbg!(&e);
        });
    Ok(())
}

pub async fn get_computers() -> anyhow::Result<Vec<model::DbComputer>> {
    let mut pool = get_sql_connection().await?;
    let recs = sqlx::query_as::<_, model::DbComputer>(
        query_select::SELECT_COMPUTER_INFORMATION_WITH_LAST_USER,
    )
    .fetch_all(&mut pool)
    .await
    // .inspect(|s| {
    //     dbg!(s);
    // })
    .inspect_err(|e| {
        dbg!(e);
    })
    .unwrap_or_default();
    Ok(recs)
}

pub async fn get_user_computers(serial_number: &str) -> anyhow::Result<Vec<model::DbLastUser>> {
    let mut pool = get_sql_connection().await?;
    let all = sqlx::query_as::<_,model::DbLastUser>("
        select (select name from users where users.id = has.user_id )  as usuario , date_begin, date_end 
        FROM has
        WHERE has.computer_id = (select id from equipaments where equipaments.serialnumber = ?1)
        order by has.date_begin desc                
        ")
        .bind(serial_number)
        .fetch_all(&mut pool)
        .await
        // .inspect(|ok| {dbg!(ok);})
        .inspect_err(|err| {dbg!(err);})?;
    Ok(all)
}
pub async fn update_user_equipament(
    actual_user: String,
    future_user: String,
    equipament: String,
) -> anyhow::Result<()> {
    let mut pool = get_sql_connection().await?;
    dbg!(&actual_user);
    dbg!(&future_user);
    dbg!(&equipament);
    let today = chrono::Local::now().date_naive().to_string();
    sqlx::query(query_select::UPDATE_LAST_USER_COMPUTER)
        .bind(&today)
        .bind(&actual_user)
        .bind(&equipament)
        .execute(&mut pool)
        .await
        .inspect(|ok| {
            dbg!(ok);
        })
        .inspect_err(|err| {
            dbg!(err);
        })
        .ok();
    sqlx::query(query_select::INSERT_NEW_USER_COMPUTER)
        .bind(&equipament)
        .bind(&future_user)
        .bind(&today)
        .execute(&mut pool)
        .await
        .inspect(|ok| {
            dbg!(ok);
        })
        .inspect_err(|err| {
            dbg!(err);
        })
        .ok();
    Ok(())
}
