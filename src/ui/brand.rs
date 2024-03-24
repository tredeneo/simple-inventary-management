use std::rc::Rc;

use slint::{ComponentHandle, StandardListViewItem, VecModel};

use crate::database;

use crate::{App, GlobalBrand};
pub async fn get_brand_list() -> anyhow::Result<Rc<VecModel<StandardListViewItem>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = database::get_brands().await?;

    for i in tmp {
        let item: StandardListViewItem = slint::format!("{}", i.name).into();
        row_data.push(item);
    }
    Ok(row_data)
}
pub async fn brand(app: &App) -> anyhow::Result<()> {
    async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_brand_list().await?;
        app.global::<GlobalBrand>()
            .set_row_data(row_data.clone().into());
        Ok(())
    }
    let myapp = app.clone_strong();
    ui_update(&myapp).await?;
    app.global::<GlobalBrand>().on_add_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            let _ = database::insert_brand(value.to_string()).await;
            let _ = ui_update(&local_app).await;
        });
    });
    let myapp = app.clone_strong();
    app.global::<GlobalBrand>().on_delete_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            let _ = database::delete_brand(value.text.to_string()).await;
            let _ = ui_update(&local_app).await;
        });
    });
    Ok(())
}
