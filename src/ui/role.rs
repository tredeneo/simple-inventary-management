use std::rc::Rc;

use slint::{ComponentHandle, StandardListViewItem, VecModel};

use crate::database;
use crate::{App, GlobalRole};

pub async fn get_role_list() -> anyhow::Result<Rc<VecModel<StandardListViewItem>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = database::get_role().await?;
    for i in tmp {
        let item: StandardListViewItem = slint::format!("{}", i.name).into();
        row_data.push(item);
    }
    Ok(row_data)
}
pub async fn role(app: &App) -> anyhow::Result<()> {
    async fn update(app: &App) -> anyhow::Result<()> {
        let row_data = get_role_list().await?;
        app.global::<GlobalRole>()
            .set_row_data(row_data.clone().into());
        Ok(())
    }
    let myapp = app.clone_strong();
    update(&myapp).await?;
    app.global::<GlobalRole>().on_add_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            let _ = database::insert_role(value.to_string()).await;
            let _ = update(&local_app).await;
        });
    });
    let myapp = app.clone_strong();
    app.global::<GlobalRole>().on_delete_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            let _ = database::delete_role(value.text.to_string()).await;
            let _ = update(&local_app).await;
        });
    });
    Ok(())
}
