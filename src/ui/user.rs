use std::rc::Rc;

use crate::database;
use slint::{ComponentHandle, ModelRc, SharedString, StandardListViewItem, VecModel};

use crate::{App, UserDetail, Users};
pub async fn get_user_list() -> anyhow::Result<Rc<VecModel<ModelRc<StandardListViewItem>>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = database::get_users().await?;
    for i in tmp {
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.name).into());
        items.push(slint::format!("{}", i.login).into());
        items.push(slint::format!("{}", i.email).into());
        row_data.push(items.into());
    }
    Ok(row_data)
}

async fn get_departs() -> anyhow::Result<ModelRc<StandardListViewItem>> {
    let depart = database::get_department().await?;
    let mut row_data = Vec::default();
    for i in depart {
        let mut item = StandardListViewItem::default();
        item.text = slint::format!("{}", i.name);
        row_data.push(item);
    }
    Ok(ModelRc::from(row_data.as_slice()))
}

async fn get_roles() -> anyhow::Result<ModelRc<StandardListViewItem>> {
    let depart = database::get_role().await?;
    let mut row_data = Vec::default();
    for i in depart {
        let mut item = StandardListViewItem::default();
        item.text = slint::format!("{}", i.name);
        row_data.push(item);
    }
    Ok(ModelRc::from(row_data.as_slice()))
}
pub async fn user_list(app: &App) -> anyhow::Result<()> {
    let row_data = get_user_list().await?;
    app.global::<Users>().set_row_data(row_data.clone().into());
    app.global::<UserDetail>()
        .set_departments(get_departs().await?);
    app.global::<UserDetail>().set_roles(get_roles().await?);
    Ok(())
}

pub async fn user_detail(app: &App) {
    let myapp = app.clone_strong();

    app.global::<UserDetail>().on_update(move || {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move { user_list(&local_app).await.unwrap() });
    });
    let myapp = app.clone_strong();
    app.global::<UserDetail>().on_create(move || {
        let local_app = myapp.clone_strong();
        let detail = myapp.global::<UserDetail>();
        let user = database::model::DbUser {
            name: detail.get_name().to_string(),
            login: detail.get_login().to_string(),
            email: detail.get_email().to_string(),
            department: detail.get_department().to_string(),
            role: detail.get_role().to_string(),
            document: detail.get_document().to_string(),
            id: detail.get_id(),
        };
        let tmp = user.clone();
        let _ = slint::spawn_local(async move {
            let _ = database::create_user(tmp).await;
            let _ = user_list(&local_app).await;
        });
    });
    let myapp = app.clone_strong();
    app.global::<UserDetail>().on_save(move || {
        let local_app = myapp.clone_strong();
        let detail = myapp.global::<UserDetail>();
        let user = database::model::DbUser {
            name: detail.get_name().to_string(),
            login: detail.get_login().to_string(),
            email: detail.get_email().to_string(),
            id: i32::default(),
            document: String::default(),
            role: String::default(),
            department: String::default(),
        };
        let tmp = user.clone();
        let _ = slint::spawn_local(async move {
            let _ = database::update_user(tmp).await;
            let _ = user_list(&local_app).await;
        });
    });
}
