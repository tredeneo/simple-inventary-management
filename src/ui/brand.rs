use crate::{database, global_update};

use crate::{App, GlobalBrand};

use slint::ComponentHandle;
pub async fn brand(app: &App) -> anyhow::Result<()> {
    let myapp = app.clone_strong();
    app.global::<GlobalBrand>().on_add_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            database::insert_brand(value.to_string()).await.ok();
            global_update(&local_app).await.ok();
        });
    });
    let myapp = app.clone_strong();
    app.global::<GlobalBrand>().on_delete_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            database::delete_brand(value.text.to_string()).await.ok();
            global_update(&local_app).await.ok();
        });
    });
    Ok(())
}
