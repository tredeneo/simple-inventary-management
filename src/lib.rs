pub mod database;
pub mod ui;
slint::include_modules!();

mod cpu {
    use crate::database;
    use crate::{App, GlobalCPU};
    use slint::{ComponentHandle, ModelRc, StandardListViewItem, VecModel};
    use std::rc::Rc;
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

    pub async fn ui_update(app: &App) -> anyhow::Result<()> {
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
}
mod gpu {

    use std::rc::Rc;

    use crate::{database, App, GlobalGPU};
    use slint::{ComponentHandle, ModelRc, StandardListViewItem, VecModel};
    async fn get_gpu_list() -> anyhow::Result<Rc<VecModel<slint::ModelRc<StandardListViewItem>>>> {
        let row_data = Rc::new(VecModel::default());
        let tmp = database::get_gpus().await?;

        for i in tmp {
            let items = Rc::new(VecModel::default());
            items.push(slint::format!("{}", i.name).into());
            items.push(slint::format!("{}", i.brand).into());
            row_data.push(items.into());
        }
        Ok(row_data)
    }

    pub async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_gpu_list().await?;
        app.global::<GlobalGPU>()
            .set_row_data(row_data.clone().into());

        let brands = database::get_brands().await?;
        let mut row_data = Vec::default();
        for i in brands {
            let item = slint::format!("{}", i.name);
            row_data.push(item)
        }
        app.global::<GlobalGPU>()
            .set_brands(ModelRc::from(row_data.as_slice()));
        Ok(())
    }
}
mod brand {
    use std::rc::Rc;

    use crate::database;

    use crate::{App, GlobalBrand};
    use slint::{ComponentHandle, StandardListViewItem, VecModel};
    async fn get_brand_list() -> anyhow::Result<Rc<VecModel<StandardListViewItem>>> {
        let row_data = Rc::new(VecModel::default());
        let tmp = database::get_brands().await?;

        for i in tmp {
            let item: StandardListViewItem = slint::format!("{}", i.name).into();
            row_data.push(item);
        }
        Ok(row_data)
    }

    pub async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_brand_list().await?;
        app.global::<GlobalBrand>()
            .set_row_data(row_data.clone().into());
        Ok(())
    }
}
mod department {

    use std::rc::Rc;

    use slint::{ComponentHandle, StandardListViewItem, VecModel};

    use crate::database;
    use crate::{App, GlobalDepartment};

    async fn get_department_list() -> anyhow::Result<Rc<VecModel<StandardListViewItem>>> {
        let row_data = Rc::new(VecModel::default());
        let tmp = database::get_department().await?;
        for i in tmp {
            let item: StandardListViewItem = slint::format!("{}", i.name).into();
            row_data.push(item);
        }
        Ok(row_data)
    }

    pub async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_department_list().await?;
        app.global::<GlobalDepartment>()
            .set_row_data(row_data.clone().into());
        Ok(())
    }
}

mod equipament_model {

    use std::rc::Rc;

    use crate::database;
    use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};

    use crate::{App, GlobalEquipamentModel, GlobalEquipamentModelDetail};
    pub async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_equipament_list().await?;

        app.global::<GlobalEquipamentModel>()
            .set_row_data(row_data.clone().into());

        update_features(app);
        Ok(())
    }

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
}
mod equipament {

    use std::rc::Rc;

    use crate::database::{self};
    use crate::{App, ComputerDetail, Computers};
    use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};

    pub async fn get_equipament_list(
    ) -> anyhow::Result<Rc<VecModel<slint::ModelRc<StandardListViewItem>>>> {
        let row_data = Rc::new(VecModel::default());
        let tmp = database::get_computers().await.unwrap_or_default();
        for i in tmp {
            let items = Rc::new(VecModel::default());
            items.push(slint::format!("{}", i.serialnumber).into());
            items.push(slint::format!("{}", i.actual_user).into());
            items.push(slint::format!("{}", i.model).into());
            row_data.push(items.into());
        }

        Ok(row_data)
    }

    async fn get_equipament_model() -> anyhow::Result<ModelRc<SharedString>> {
        let cpus = database::get_equipament_model().await?;
        let mut row_data = Vec::default();
        for i in cpus {
            row_data.push(slint::format!("{}", i.name));
        }
        Ok(ModelRc::from(row_data.as_slice()))
    }
    pub async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_equipament_list().await?;

        app.global::<Computers>()
            .set_row_data(row_data.clone().into());

        app.global::<ComputerDetail>()
            .set_model_equipaments(get_equipament_model().await?);
        Ok(())
    }
}
mod user {

    use std::rc::Rc;

    use crate::database::{self, get_department};
    use slint::{ComponentHandle, ModelRc, StandardListViewItem, VecModel};

    use crate::{App, ChangeEquipament, UserDetail, Users};
    pub async fn get_user_list() -> anyhow::Result<Rc<VecModel<ModelRc<StandardListViewItem>>>> {
        let row_data = Rc::new(VecModel::default());
        let tmp = database::get_users().await?;
        for i in tmp {
            let items = Rc::new(VecModel::default());
            items.push(slint::format!("{}", i.name.to_lowercase()).into());
            items.push(slint::format!("{}", i.login).into());
            items.push(slint::format!("{}", i.email).into());

            row_data.push(items.into());
        }
        Ok(row_data)
    }

    pub async fn ui_update(app: &App) -> anyhow::Result<()> {
        let row_data = get_user_list().await?;
        app.global::<Users>().set_row_data(row_data.clone().into());
        app.global::<ChangeEquipament>()
            .set_users(row_data.clone().into());
        let tmp = get_department().await?;
        let row_data = Rc::new(VecModel::default());
        for i in tmp {
            row_data.push(slint::format!("{}", i.name));
        }
        app.global::<UserDetail>().set_departments(row_data.into());
        Ok(())
    }
}

pub async fn global_update(app: &App) -> anyhow::Result<()> {
    cpu::ui_update(app).await?;
    gpu::ui_update(app).await?;
    brand::ui_update(app).await?;
    department::ui_update(app).await?;
    equipament_model::ui_update(app).await?;
    equipament::ui_update(app).await?;
    user::ui_update(app).await?;
    Ok(())
}
