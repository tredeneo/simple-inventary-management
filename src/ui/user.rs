use std::rc::Rc;

use crate::database;
use slint::{ComponentHandle, StandardListViewItem, VecModel};

use crate::{App, UserDetail, Users};
pub async fn get_user_list() -> anyhow::Result<Rc<VecModel<slint::ModelRc<StandardListViewItem>>>> {
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
pub async fn user_list(app: &App) -> anyhow::Result<()> {
    let row_data = get_user_list().await?;

    app.global::<Users>().set_row_data(row_data.clone().into());
    Ok(())
}

pub async fn user_detail_update(app: &App) {
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
            let _ = user_list(&local_app).await;
        });
    });
}
