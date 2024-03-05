#![windows_subsystem = "windows"]

slint::include_modules!();
use chrono;
use slint::{ComponentHandle, StandardListViewItem, VecModel};
use sqlx::sqlite::SqlitePool;
use std::rc::Rc;

#[derive(sqlx::FromRow, Debug, Default, Clone)]
struct DbUser {
    name: String,
    login: String,
    email: String,
}
async fn pegar_computador() -> anyhow::Result<Vec<DbUser>> {
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
    let tmp = pegar_computador().await?;
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

async fn ui_equipament_list(app: &App) -> anyhow::Result<()> {
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    let tmp = pegar_computador().await?;
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
#[derive(sqlx::FromRow, Debug, Default, Clone)]
struct DbHas {
    user: String,
    computer: String,
    begin: String,
    end: String,
}

async fn update_user_equipament(equipament: String) -> anyhow::Result<()> {
    dbg!(&equipament);
    let poll = SqlitePool::connect("banco.sqlite3").await?;

    let today = chrono::Local::now().date_naive().to_string();
    let recs = sqlx::query_as::<_, DbHas>(
        r#"
            select * from has -- update has
            -- set data_fim=?1 
            where computer_id=?2 and user_id=?3 and data_fim=NULL
        "#,
    )
    // .bind(user)
    .bind(equipament)
    // .execute(&poll)
    .fetch_all(&poll)
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
        let _ = slint::spawn_local(async move {
            let _ = update_user_equipament(serial.to_string()).await;
        });
        // dbg!(today);
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let myapp = App::new().unwrap();
    let _ = ui_user_list(&myapp).await;
    let _ = ui_user_detail_update(&myapp).await;
    let _ = ui_change_equipament(&myapp).await;

    myapp.run().unwrap();
    Ok(())
}
