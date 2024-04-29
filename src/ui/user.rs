use std::rc::Rc;

use crate::database;
use slint::{ComponentHandle, Model, ModelRc, StandardListViewItem, VecModel};

use crate::{App, UserDetail, Users};
pub async fn get_user_list() -> anyhow::Result<Rc<VecModel<ModelRc<StandardListViewItem>>>> {
    let row_data = Rc::new(VecModel::default());
    let tmp = database::get_users().await?;
    for i in tmp {
        dbg!(&i);
        let items = Rc::new(VecModel::default());
        items.push(slint::format!("{0}", i.name).into());
        items.push(slint::format!("{}", i.login).into());
        items.push(slint::format!("{}", i.email).into());

        row_data.push(items.into());
    }
    Ok(row_data)
}

fn update_departments(app: &App) {
    let myapp = app.clone_strong();
    let _ = slint::spawn_local(async move {
        myapp
            .global::<UserDetail>()
            .set_departments(get_departs().await.unwrap_or_default());
    });
}

async fn filter_department(app: &App) {
    let myapp = app.clone_strong();

    app.global::<UserDetail>()
        .on_filter_departments(move |text| {
            let local_app = myapp.clone_strong();
            update_departments(&local_app);
            let filted: Vec<StandardListViewItem> = local_app
                .global::<UserDetail>()
                .get_departments()
                .iter()
                .filter(|arg| {
                    arg.text
                        .to_lowercase()
                        .contains(&text.clone().to_lowercase().as_str())
                })
                .collect();
            let tmp = Rc::new(VecModel::from(filted)).into();
            local_app.global::<UserDetail>().set_departments(tmp)
        });
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

pub async fn user_list(app: &App) -> anyhow::Result<()> {
    let row_data = get_user_list().await?;

    dbg!("Oi");
    app.global::<Users>().set_row_data(row_data.clone().into());

    update_departments(&app);
    Ok(())
}

pub async fn user_detail(app: &App) {
    let myapp = app.clone_strong();

    let _ = filter_department(&app).await;
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
            document: detail.get_document().to_string(),
            id: detail.get_id(),
            extension: detail.get_extension().to_string(),
            phone_number: detail.get_phone_number().to_string(),
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
            department: String::default(),
            extension: detail.get_extension().to_string(),
            phone_number: detail.get_phone_number().to_string(),
        };

        let tmp = user.clone();
        let _ = slint::spawn_local(async move {
            let _ = database::update_user(tmp).await;
            let _ = user_list(&local_app).await;
        });
    });
}
