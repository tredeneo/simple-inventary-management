#![windows_subsystem = "windows"]

slint::include_modules!();

use simple_inventary::database::{self, get_brands, get_cpus};
use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};
use std::rc::Rc;

async fn get_user_list() -> anyhow::Result<Rc<VecModel<slint::ModelRc<StandardListViewItem>>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = database::get_users().await?;
    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.name).into());
        items.push(slint::format!("{}", i.login).into());
        items.push(slint::format!("{}", i.email).into());
        items.push(slint::format!("{}", i.id).into());
        row_data.push(items.into());
    }
    Ok(row_data)
}

async fn ui_user_list(app: &App) -> anyhow::Result<()> {
    let row_data = get_user_list().await?;

    app.global::<Users>().set_row_data(row_data.clone().into());
    Ok(())
}

async fn ui_user_detail_update(app: &App) {
    let myapp = app.clone_strong();
    app.global::<UserDetail>().on_save(move || {
        let local_app = myapp.clone_strong();
        let detail = myapp.global::<UserDetail>();
        let user = database::model::DbUser {
            name: detail.get_name().to_string(),
            login: detail.get_login().to_string(),
            email: detail.get_email().to_string(),
            id: detail.get_id(),
        };
        let tmp = user.clone();
        let _ = slint::spawn_local(async move {
            let _ = database::update_user(tmp).await;
            let _ = ui_user_list(&local_app).await;
        });
    });
}

async fn ui_equipament_list(app: &App) -> anyhow::Result<()> {
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    let tmp = database::get_computers().await?;
    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.serialnumber).into());
        items.push(slint::format!("{}", i.brand).into());
        items.push(slint::format!("{}", i.actual_user).into());
        items.push(slint::format!("{}", i.model).into());
        row_data.push(items.into());
    }

    app.global::<Computers>()
        .set_row_data(row_data.clone().into());
    Ok(())
}

async fn ui_change_equipament(app: &App) -> anyhow::Result<()> {
    let myapp = app.clone_strong();

    let row_data = get_user_list().await?;
    app.global::<ChangeEquipament>()
        .set_users(row_data.clone().into());

    app.global::<ChangeEquipament>().on_change_user(move || {
        let local_app = myapp.clone_strong();
        let computer = myapp.global::<ComputerDetail>();
        let serial = computer.get_serial_number();

        let actual_user = computer.get_actual_user();
        let future_user = local_app.global::<ChangeEquipament>().get_future_user();

        let _ = slint::spawn_local(async move {
            let _ = database::update_user_equipament(
                actual_user.to_string(),
                future_user.to_string(),
                serial.to_string(),
            )
            .await;
            let _ = ui_equipament_list(&local_app).await;
        });
    });
    Ok(())
}

async fn get_brand_list() -> anyhow::Result<Rc<VecModel<StandardListViewItem>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = get_brands().await?;

    for i in tmp {
        let item: StandardListViewItem = slint::format!("{}", i.name).into();
        row_data.push(item);
    }
    Ok(row_data)
}
async fn ui_brand(app: &App) -> anyhow::Result<()> {
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

async fn get_cpu_list() -> anyhow::Result<Rc<VecModel<slint::ModelRc<StandardListViewItem>>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = get_cpus().await?;

    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{}", i.name).into());
        items.push(slint::format!("{}", i.brand).into());
        row_data.push(items.into());
    }
    Ok(row_data)
}

async fn ui_cpu(app: &App) -> anyhow::Result<()> {
    async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_cpu_list().await?;
        app.global::<GlobalCPU>()
            .set_row_data(row_data.clone().into());

        let brands = get_brands().await?;
        let mut row_data = Vec::default();
        for i in brands {
            let item: SharedString = slint::format!("{}", i.name).into();
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
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let myapp = App::new().unwrap();
    let _ = ui_user_list(&myapp).await;
    let _ = ui_user_detail_update(&myapp).await;
    let _ = ui_change_equipament(&myapp).await;
    let _ = ui_equipament_list(&myapp).await;
    let _ = ui_brand(&myapp).await;
    let _ = ui_cpu(&myapp).await;

    myapp.run().unwrap();
    Ok(())
}
