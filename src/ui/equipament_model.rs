use std::rc::Rc;

use crate::database;
use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};

use crate::{App, GlobalEquipamentModel, GlobalEquipamentModelDetail};
async fn get_equipament_list() -> anyhow::Result<Rc<VecModel<ModelRc<StandardListViewItem>>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = database::get_equipament_model().await?;
    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.name.to_lowercase()).into());
        items.push(slint::format!("{}", i.brand).into());
        items.push(slint::format!("{}", i.cpu).into());

        row_data.push(items.into());
    }
    Ok(row_data)
}

fn update_features(app: &App) {
    let myapp = app.clone_strong();
    let _ = slint::spawn_local(async move {
        let tmp = myapp.global::<GlobalEquipamentModelDetail>();
        tmp.set_brands(get_brands().await.unwrap_or_default());
        tmp.set_cpus(get_cpu().await.unwrap_or_default());
        tmp.set_gpus(get_gpu().await.unwrap_or_default());
    });
}

async fn get_gpu() -> anyhow::Result<ModelRc<SharedString>> {
    let depart = database::get_gpus().await?;
    let mut row_data = Vec::default();
    for i in depart {
        let item = slint::format!("{}", i.name);
        row_data.push(item);
    }
    Ok(ModelRc::from(row_data.as_slice()))
}
async fn get_cpu() -> anyhow::Result<ModelRc<SharedString>> {
    let depart = database::get_cpus().await?;
    let mut row_data = Vec::default();
    for i in depart {
        let item = slint::format!("{}", i.name);
        row_data.push(item);
    }
    Ok(ModelRc::from(row_data.as_slice()))
}
async fn get_brands() -> anyhow::Result<ModelRc<SharedString>> {
    let depart = database::get_brands().await?;
    let mut row_data = Vec::default();
    for i in depart {
        let item = slint::format!("{}", i.name);
        row_data.push(item);
    }
    Ok(ModelRc::from(row_data.as_slice()))
}

pub async fn equipament_list(app: &App) -> anyhow::Result<()> {
    let row_data = get_equipament_list().await?;

    app.global::<GlobalEquipamentModel>()
        .set_row_data(row_data.clone().into());

    update_features(app);
    Ok(())
}

pub async fn equipament_detail(app: &App) {
    let myapp = app.clone_strong();

    app.global::<GlobalEquipamentModelDetail>()
        .on_update(move || {
            let local_app = myapp.clone_strong();
            let _ = slint::spawn_local(async move { equipament_list(&local_app).await.unwrap() });
        });
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
                };
                async move {
                    let _ = database::insert_equipament_model(tmp).await;
                    let _ = equipament_list(&local_app).await;
                }
            });
            detail.set_brand(SharedString::default());
            detail.set_cpu(SharedString::default());
            detail.set_gpu(SharedString::default());
            detail.set_name(SharedString::default());
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
                    // let brand = database::get_brand_by_name(detail.get_brand().to_string())
                    //     .await
                    //     .unwrap()
                    //     .id
                    //     .to_string();

                    // let cpu = database::get_cpu_by_name(detail.get_cpu().to_string())
                    //     .await
                    //     .unwrap()
                    //     .id
                    //     .to_string();

                    // let gpu = database::get_gpu_by_name(detail.get_gpu().to_string())
                    //     .await
                    //     .unwrap()
                    //     .id
                    //     .to_string();

                    let equipament = database::model::DbEquipamentModel {
                        name: detail.get_name().to_string(),
                        brand: detail.get_brand().to_string(),
                        cpu: detail.get_cpu().to_string(),
                        gpu: detail.get_gpu().to_string(),
                    };
                    let tmp = equipament.clone();
                    let _ = database::update_equipament_model(tmp).await;
                    let _ = equipament_list(&local_app).await;
                }
            });
        });
}
