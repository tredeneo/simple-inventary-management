use std::rc::Rc;

use crate::{database, global_update};
use slint::{ComponentHandle, SharedString, VecModel};

use crate::{App, UserDetail, Users};

pub async fn user_detail(app: &App) {
    let myapp = app.clone_strong();

    app.global::<UserDetail>().on_update(move || {
        let local_app = myapp.clone_strong();
        slint::spawn_local(async move { global_update(&local_app).await.ok() }).ok();
    });
    let myapp = app.clone_strong();
    app.global::<UserDetail>().on_create(move || {
        let local_app = myapp.clone_strong();
        let detail = myapp.global::<UserDetail>();
        let user = database::model::DbUser {
            name: detail.get_name().trim().to_string(),
            login: detail.get_login().trim().to_string(),
            email: detail.get_email().trim().to_string(),
            department: detail.get_department().trim().to_string(),
            document: detail.get_document().trim().to_string(),
            id: 0,
            extension: detail.get_extension().trim().to_string(),
            phone_number: detail.get_phone_number().trim().to_string(),
        };
        let tmp = user.clone();
        let _ = slint::spawn_local(async move {
            database::create_user(tmp).await.ok();
            global_update(&local_app).await.ok();
        });
        detail.set_name(SharedString::default());
        detail.set_login(SharedString::default());
        detail.set_extension(SharedString::default());
        detail.set_email(SharedString::new());
        detail.set_department(SharedString::new());
        detail.set_document(SharedString::new());
        detail.set_id(0.to_string().into());
        detail.set_phone_number(SharedString::new());
    });

    let myapp = app.clone_strong();
    app.global::<Users>().on_select_user(move |user_name| {
        let local_app = myapp.clone_strong();
        let _ = slint::spawn_local(async move {
            let user_detail = local_app.global::<UserDetail>();
            let user = database::get_specific_user_by_name(user_name.to_string())
                .await
                .unwrap_or_default();
            let tmp = database::get_department_by_id(user.department.to_string())
                .await
                .unwrap_or_default();

            user_detail.set_name(user.name.into());
            user_detail.set_department(tmp.name.into());
            user_detail.set_document(user.document.into());
            user_detail.set_email(user.email.into());
            user_detail.set_extension(user.extension.into());
            user_detail.set_login(user.login.clone().into());
            user_detail.set_phone_number(user.phone_number.into());

            let row_data = Rc::new(VecModel::default());
            let historic = database::get_equipaments_by_users(user.login.to_string())
                .await
                .unwrap_or_default();
            for i in historic {
                let items = Rc::new(VecModel::default());
                items.push(slint::format!("{}", i.brand).into());
                items.push(slint::format!("{}", i.model).into());
                items.push(slint::format!("{}", i.serialnumber).into());
                items.push(slint::format!("{}", i.initial_date).into());
                items.push(slint::format!("{}", i.final_date).into());
                row_data.push(items.into());
            }
            user_detail.set_row_data(row_data.into());
            global_update(&local_app).await.ok();
        });
    });

    let myapp = app.clone_strong();
    app.global::<UserDetail>().on_save(move || {
        let local_app = myapp.clone_strong();

        let _ = slint::spawn_local({
            let user_app = myapp.clone_strong();
            async move {
                let detail = user_app.global::<UserDetail>();
                let tmp = database::get_department_by_name(detail.get_department().to_string())
                    .await
                    .unwrap()
                    .id;
                let user = database::model::DbUser {
                    name: detail.get_name().trim().to_string(),
                    login: detail.get_login().trim().to_string(),
                    email: detail.get_email().trim().to_string(),
                    id: tmp,
                    document: detail.get_document().trim().to_string(),
                    department: detail.get_department().trim().to_string(),
                    extension: detail.get_extension().trim().to_string(),
                    phone_number: detail.get_phone_number().trim().to_string(),
                };
                let tmp = user.clone();
                database::update_user(tmp).await.ok();
                global_update(&local_app).await.ok();
            }
        });
    });
}
