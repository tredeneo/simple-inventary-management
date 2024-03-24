use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, StandardListViewItem, VecModel};

use crate::{database, App, GlobalCPU};

async fn get_cpu_list() -> anyhow::Result<Rc<VecModel<slint::ModelRc<StandardListViewItem>>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = database::get_cpus().await?;

    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{}", i.name).into());
        items.push(slint::format!("{}", i.brand).into());
        row_data.push(items.into());
    }
    Ok(row_data)
}

pub async fn cpu(app: &App) -> anyhow::Result<()> {
    async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_cpu_list().await?;
        app.global::<GlobalCPU>()
            .set_row_data(row_data.clone().into());

        let brands = database::get_brands().await?;
        let mut row_data = Vec::default();
        for i in brands {
            let item = slint::format!("{}", i.name);
            row_data.push(item)
        }
        app.global::<GlobalCPU>()
            .set_brands(ModelRc::from(row_data.as_slice()));
        Ok(())
    }
    let myapp = app.clone_strong();
    ui_update(&myapp).await?;
    app.global::<GlobalCPU>().on_add_item(move |name, brand| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            let _ = database::insert_cpu(name.to_string(), brand.to_string()).await;
            let _ = ui_update(&local_app).await;
        });
    });
    let myapp = app.clone_strong();
    app.global::<GlobalCPU>().on_delete_item(move |value| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            let _ = database::delete_cpu(value.text.to_string()).await;
            let _ = ui_update(&local_app).await;
        });
    });
    Ok(())
}
