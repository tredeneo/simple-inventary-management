slint::include_modules!();
use slint::{Model, SharedString, StandardListViewItem, VecModel};
use sqlx::{sqlite::SqlitePool, Row};
use std::rc::Rc;

#[derive(sqlx::FromRow, Debug, Default)]
struct DbUser {
    name: String,
    login: String,
    setor: String,
}
async fn pegar_computador() -> anyhow::Result<Vec<DbUser>> {
    let pool = SqlitePool::connect("banco.sqlite3").await?;
    let recs = sqlx::query_as::<_, DbUser>(
        r#"
            select name,login,setores.setor from users
            join setores on setores.id  = users.setor 

        "#,
    )
    .fetch_all(&pool)
    .await?;
    // dbg!(recs);
    // let mut itens = Vec::new();
    // for rec in recs {
    //     let model = rec.try_get("name").unwrap_or_default();
    //     itens.push(model);
    // }
    // Ok(itens)
    Ok(recs)
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let myapp = App::new().unwrap();
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    let tmp = pegar_computador().await?;
    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.name).into());
        items.push(slint::format!("{}", i.login).into());
        items.push(slint::format!("{}", i.setor).into());
        row_data.push(items.into());
    }
    myapp
        .global::<Users>()
        .set_row_data(row_data.clone().into());

    myapp.run().unwrap();
    Ok(())
}
