use std::rc::Rc;

use crate::database::{self};
use crate::{App, ChangeEquipament, ComputerDetail, Computers};
use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};

pub async fn equipament_list(app: &App) -> anyhow::Result<()> {
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

async fn get_brands() -> anyhow::Result<ModelRc<SharedString>> {
    let brands = database::get_brands().await?;
    let mut row_data = Vec::default();
    for i in brands {
        row_data.push(slint::format!("{}", i.name));
    }
    Ok(ModelRc::from(row_data.as_slice()))
}

async fn get_cpus() -> anyhow::Result<ModelRc<SharedString>> {
    let cpus = database::get_cpus().await?;
    let mut row_data = Vec::default();
    for i in cpus {
        row_data.push(slint::format!("{}", i.name));
    }
    Ok(ModelRc::from(row_data.as_slice()))
}

async fn get_equipament_model() -> anyhow::Result<ModelRc<SharedString>> {
    let cpus = database::get_equipament_model().await?;
    let mut row_data = Vec::default();
    for i in cpus {
        row_data.push(slint::format!("{}", i.name));
    }
    Ok(ModelRc::from(row_data.as_slice()))
}

async fn get_gpus() -> anyhow::Result<ModelRc<SharedString>> {
    let gpus = database::get_gpus().await?;
    let mut row_data = Vec::default();
    for i in gpus {
        row_data.push(slint::format!("{}", i.name));
    }
    Ok(ModelRc::from(row_data.as_slice()))
}
pub async fn change_equipament(app: &App) -> anyhow::Result<()> {
    use crate::ui::user::get_user_list;
    let myapp = app.clone_strong();

    let row_data = get_user_list().await?;
    app.global::<ChangeEquipament>()
        .set_users(row_data.clone().into());

    app.global::<ComputerDetail>()
        .set_brands(get_brands().await?);

    app.global::<ComputerDetail>().set_cpus(get_cpus().await?);

    app.global::<ComputerDetail>().set_gpus(get_gpus().await?);

    app.global::<ComputerDetail>()
        .set_model_equipaments(get_equipament_model().await?);

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
            let _ = equipament_list(&local_app).await;
        });
    });
    Ok(())
}
