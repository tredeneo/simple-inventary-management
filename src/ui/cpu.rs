use slint::ComponentHandle;

use crate::{database, global_update, App, GlobalCPU};

pub async fn cpu(app: &App) -> anyhow::Result<()> {
    let myapp = app.clone_strong();
    app.global::<GlobalCPU>().on_add_item(move |name, brand| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            database::insert_cpu(name.to_string(), brand.to_string())
                .await
                .ok();
            global_update(&local_app).await.ok();
        });
    });
    let myapp = app.clone_strong();
    app.global::<GlobalCPU>().on_delete_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            database::delete_cpu(value.text.to_string()).await.ok();
            global_update(&local_app).await.ok();
        });
    });
    Ok(())
}
