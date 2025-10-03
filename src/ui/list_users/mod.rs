use cosmic::{
    Action, Element,
    iced::{Alignment, Length},
    widget::text,
};

// mod detail;
mod detail;
pub mod list;

#[derive(Default)]
enum View {
    #[default]
    ListUsers,
    DetailUser(detail::UserDetailPage),
}

#[derive(Debug, Clone)]
enum UsersTabMessage {
    ListUsers,
    DetailUser,
}

pub struct UsersTab {
    // screen: list::ListUsers,
    view: View,
}

impl UsersTab {
    fn view(&self) -> Element<'_, UsersTabMessage> {
        use cosmic::iced::widget::{button, column, row};
        use cosmic::widget::container;
        match &self.view {
            View::ListUsers => {
                let buttons = row![button("back")]; // .on_press(UsersMessage::CloseDetail)];

                let coluna = column![
                    buttons,
                    text("hello world").size(32),
                    // text(format!("{}", tmp.name)).size(32),
                    // text(tmp.department).size(30),
                    // text(tmp.email).size(30)
                ];
                container(coluna)
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .into()
            }

            View::DetailUser(page) => page
                .view()
                .map(|arg| UsersTabMessage::DetailUser(Action::App(arg))),
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum UsersMessage {
//     List(list::ListUser),
//     // Futuramente: Detail(...)
// }
