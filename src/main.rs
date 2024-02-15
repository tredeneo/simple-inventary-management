slint::include_modules!();
use slint::{StandardListViewItem, VecModel};
use sqlx::{sqlite::SqlitePool, Row};
use std::rc::Rc;

async fn pegar_computador() -> anyhow::Result<Vec<String>> {
    let pool = SqlitePool::connect("banco.sqlite3").await?;
    let recs = sqlx::query(
        r#"
            select * from 'computer'
        "#,
    )
    .fetch_all(&pool)
    .await?;
    let mut itens = Vec::new();
    for rec in recs {
        let model = rec.try_get("model").unwrap_or_default();
        itens.push(model);
    }
    Ok(itens)
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let myapp = App::new().unwrap();
    dbg!(pegar_computador().await?);
    // unreachable!();
    // let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    // let tmp = pegar_computador().await?.unwrap();
    // for i in tmp {
    //     let items = Rc::new(VecModel::default());
    //     items.push(slint::format!("{0}", i.name));
    //     row_data.push(items.into());
    // }
    // myapp
    //     .global::<Users>()
    //     .set_row_data(row_data.clone().into());

    // myapp.run().unwrap();
    Ok(())
}
