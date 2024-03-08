#![windows_subsystem = "windows"]

slint::include_modules!();
use anyhow::anyhow;
use chrono;
use slint::{ComponentHandle, StandardListViewItem, VecModel};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::rc::Rc;

type actual_database = Pool<Sqlite>;
#[derive(sqlx::FromRow, Debug, Default, Clone)]
struct DbUser {
    name: String,
    login: String,
    email: String,
}

#[derive(sqlx::FromRow, Debug, Default, Clone)]
struct DbComputer {
    serialnumber: String,
    brand: String,
    actual_user: String,
    model: String,
}
async fn get_users() -> anyhow::Result<Vec<DbUser>> {
    let pool = SqlitePool::connect("banco.sqlite3").await?;
    let recs = sqlx::query_as::<_, DbUser>(
        r#"
            select name,login,email --setores.setor 
            from users
            join setores on setores.id  = users.setor 

        "#,
    )
    .fetch_all(&pool)
    .await?;
    Ok(recs)
}

// async fn query_select<'r, T>(query: &str, db: &Pool<Sqlite>) -> anyhow::Result<Vec<T>>
// where
//     T: sqlx::FromRow<R>,
//     R: sqlx::Row,
// {
//     todo!();
//     let recs = sqlx::query_as::<_, &'r T>(
//         r#"
//             select
//         "#,
//     )
//     .fetch_all(db)
//     .await?;
//     Ok(recs)
// }

async fn get_computers(db: &actual_database) -> anyhow::Result<Vec<DbComputer>> {
    let recs = sqlx::query_as::<_, DbComputer>(
        r#"
            select  serialnumber ,brands.name as brand, model, login as actual_user
            from computer 
            join brands on computer.brand  = brands.id 
            join has on has.computer_id = computer.id 
            join users on users.id = has.user_id         
        "#,
    )
    .fetch_all(db)
    .await?;
    Ok(recs)
}

async fn update_user(user: DbUser) -> anyhow::Result<()> {
    let poll = SqlitePool::connect("banco.sqlite3").await?;
    let _ = sqlx::query(
        r#"
            update users
            set name=?1, email=?2 
            where login=?3
        "#,
    )
    .bind(user.name)
    .bind(user.email)
    .bind(user.login)
    .execute(&poll)
    .await?
    .rows_affected();
    Ok(())
}

async fn ui_user_list(app: &App) -> anyhow::Result<()> {
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    let tmp = get_users().await?;
    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.name).into());
        items.push(slint::format!("{}", i.login).into());
        items.push(slint::format!("{}", i.email).into());
        row_data.push(items.into());
    }

    app.global::<Users>().set_row_data(row_data.clone().into());
    Ok(())
}

async fn ui_user_detail_update(app: &App) {
    let myapp = app.clone_strong();
    app.global::<UserDetail>().on_save(move || {
        let local_app = myapp.clone_strong();
        let detail = myapp.global::<UserDetail>();
        let user = DbUser {
            name: detail.get_name().to_string(),
            login: detail.get_login().to_string(),
            email: detail.get_email().to_string(),
        };
        let tmp = user.clone();
        let _ = slint::spawn_local(async move {
            let _ = update_user(tmp).await;
            let _ = ui_user_list(&local_app).await;
        });
    });
}

async fn ui_equipament_list(app: &App, db: &Pool<Sqlite>) -> anyhow::Result<()> {
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    let tmp = get_computers(db).await?;
    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.serialnumber).into());
        items.push(slint::format!("{}", i.brand).into());
        items.push(slint::format!("{}", i.actual_user).into());
        items.push(slint::format!("{}", i.model).into());
        row_data.push(items.into());
    }

    app.global::<Computers>()
        .set_row_data(row_data.clone().into());
    Ok(())
}
#[derive(sqlx::FromRow, Debug, Default, Clone)]
struct DbHas {
    user: String,
    computer: String,
    begin: String,
    end: String,
}

async fn update_user_equipament(user: String, equipament: String) -> anyhow::Result<()> {
    let poll = SqlitePool::connect("banco.sqlite3").await?;
    let today = chrono::Local::now().date_naive().to_string();
    dbg!(&user);
    unimplemented!();
    let recs = sqlx::query(
        r#"
            select * from has -- update has
            -- set data_fim=?1 
            where computer_id=?2 and user_id=?3 and data_fim=NULL
        "#,
    )
    .bind(today)
    .bind(equipament)
    .bind(user)
    .execute(&poll)
    .await?;
    // .rows_affected();
    dbg!(recs);
    Ok(())
}
async fn ui_change_equipament(app: &App) {
    let myapp = app.clone_strong();
    app.global::<ChangeEquipament>().on_change_user(move || {
        let local_app = myapp.clone_strong();
        // let user = myapp.global::<UserDetail>().get_login();
        let computer = myapp.global::<ComputerDetail>();
        let serial = computer.get_serial_number();

        let brand = computer.get_brand();
        // let login = computer.get_actual_user();
        let future_user = local_app.global::<ChangeEquipament>().get_future_user();
        // let model = computer.get_model();

        // dbg!(&brand, &login, &model, &serial);
        let _ = slint::spawn_local(async move {
            let _ = update_user_equipament(future_user.to_string(), serial.to_string()).await;
        });
        // dbg!(today);
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let poll = SqlitePool::connect("banco.sqlite3").await?;
    let myapp = App::new().unwrap();
    let _ = ui_user_list(&myapp).await;
    let _ = ui_user_detail_update(&myapp).await;
    let _ = ui_change_equipament(&myapp).await;
    let _ = ui_equipament_list(&myapp, &poll).await;

    myapp.run().unwrap();
    Ok(())
}
