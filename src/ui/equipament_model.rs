use crate::{database, global_update};
use slint::{ComponentHandle, SharedString};

use crate::{App, GlobalEquipamentModel, GlobalEquipamentModelDetail};

pub async fn equipament_detail(app: &App) {
    let myapp = app.clone_strong();
    app.global::<GlobalEquipamentModelDetail>()
        .on_create(move || {
            let local_app = myapp.clone_strong();
            let detail = myapp.global::<GlobalEquipamentModelDetail>();
            let _ = slint::spawn_local({
                let tmp = myapp.global::<GlobalEquipamentModelDetail>();
                let tmp = database::model::DbEquipamentModel {
                    name: tmp.get_name().to_string(),
                    brand: tmp.get_brand().to_string(),
                    cpu: tmp.get_cpu().to_string(),
                    gpu: tmp.get_gpu().to_string(),
                    smartphone: tmp.get_smartphone() as i32,
                };
                async move {
                    database::insert_equipament_model(tmp).await.ok();
                    global_update(&local_app).await.ok();
                }
            });
            detail.set_brand(SharedString::default());
            detail.set_cpu(SharedString::default());
            detail.set_gpu(SharedString::default());
            detail.set_name(SharedString::default());
            detail.set_smartphone(false);
        });

    let myapp = app.clone_strong();
    app.global::<GlobalEquipamentModel>()
        .on_select_equipament(move |equipament_name| {
            let local_app = myapp.clone_strong();
            let _ = slint::spawn_local(async move {
                let equipament_detail = local_app.global::<GlobalEquipamentModelDetail>();
                let user = database::get_specific_equipament_model(equipament_name.to_string())
                    .await
                    .inspect_err(|e| {
                        dbg!(e);
                    })
                    .unwrap_or_default();

                equipament_detail.set_name(user.name.into());
                equipament_detail.set_brand(user.brand.into());
                equipament_detail.set_cpu(user.cpu.into());
                equipament_detail.set_gpu(user.gpu.into());
            });
        });

    let myapp = app.clone_strong();
    app.global::<GlobalEquipamentModelDetail>()
        .on_save(move || {
            let local_app = myapp.clone_strong();

            let _ = slint::spawn_local({
                let user_app = myapp.clone_strong();
                async move {
                    let detail = user_app.global::<GlobalEquipamentModelDetail>();
                    let tmp = database::model::DbEquipamentModel {
                        name: detail.get_name().to_string(),
                        brand: detail.get_brand().to_string(),
                        cpu: detail.get_cpu().to_string(),
                        gpu: detail.get_gpu().to_string(),
                        smartphone: detail.get_smartphone() as i32,
                    };
                    database::update_equipament_model(tmp).await.ok();
                    global_update(&local_app).await.ok();
                }
            });
        });
}
