use iced::{Element, Length, Renderer, Task, Theme};
use iced_table::table;

use crate::Message;
use crate::Tab;
use crate::database;
use iced::widget::{Text, button, container, responsive, scrollable, text};

pub enum ListUsersAction {
    None,
    Run(Task<ListUsersMessage>),
}
enum ColumnKind {
    Name,
    Department,
    Edit,
}

// #[derive(Default)]
pub struct ListUsers {
    columns: Vec<Column>,
    rows: Vec<Row>,
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    _theme: Theme,
}

#[derive(Debug, Clone)]
pub enum ListUsersMessage {
    SyncHeader(scrollable::AbsoluteOffset),
    Edit(usize),
    Resizing(usize, f32),
    Resized,
    GetUsers(Vec<database::model::DbUser>),
    Temp,
}

impl ListUsers {
    pub fn new() -> (Self, Task<ListUsersMessage>) {
        let screen = Self {
            columns: vec![
                Column::new(ColumnKind::Name),
                Column::new(ColumnKind::Department),
                Column::new(ColumnKind::Edit),
            ],
            rows: Vec::new(),
            header: scrollable::Id::unique(),
            body: scrollable::Id::unique(),
            footer: scrollable::Id::unique(),
            _theme: Theme::Light,
        };

        let get_users = Task::perform(database::get_users(), |arg| {
            let tmp = arg.unwrap_or_default();
            ListUsersMessage::GetUsers(tmp)
        });

        (screen, get_users)
    }
    pub fn update(&mut self, _message: ListUsersMessage) -> ListUsersAction {
        match _message {
            ListUsersMessage::SyncHeader(offset) => ListUsersAction::Run(Task::batch(vec![
                scrollable::scroll_to(self.header.clone(), offset),
                scrollable::scroll_to(self.footer.clone(), offset),
            ])),
            ListUsersMessage::Edit(index) => {
                dbg!(index);
                ListUsersAction::None
            }
            ListUsersMessage::Resizing(index, offset) => {
                if let Some(column) = self.columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
                ListUsersAction::None
            }
            ListUsersMessage::Resized => {
                self.columns.iter_mut().for_each(|column| {
                    if let Some(offset) = column.resize_offset.take() {
                        column.width += offset;
                    }
                });
                ListUsersAction::None
            }
            ListUsersMessage::Temp => {
                let tmp = Task::perform(database::get_users(), |arg| {
                    let tmp = arg.unwrap_or_default();
                    ListUsersMessage::GetUsers(tmp)
                });
                ListUsersAction::Run(tmp)
            }

            ListUsersMessage::GetUsers(db_users) => {
                self.rows = db_users
                    .into_iter()
                    .map(|i| Row {
                        name: i.name,
                        department: i.department,
                    })
                    .collect();
                ListUsersAction::None
            }
        }
    }
}
impl Tab for ListUsers {
    fn content(&self) -> iced::Element<'_, Self::Message> {
        let table = responsive(|size| {
            let mut table = table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                ListUsersMessage::SyncHeader,
            )
            .min_width(size.width)
            .on_column_resize(ListUsersMessage::Resizing, ListUsersMessage::Resized);

            table = table.footer(self.footer.clone());

            table.into()
        });

        let tmp: Element<_> = container(container(table).width(Length::Fill).height(Length::Fill))
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        tmp.map(Message::ListUsers)
    }

    type Message = Message;

    fn title(&self) -> String {
        String::from("Users List")
    }

    fn tab_label(&self) -> iced_aw::sidebar::TabLabel {
        iced_aw::sidebar::TabLabel::Text(self.title())
    }
}

struct Column {
    kind: ColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}

impl Column {
    fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            // ColumnKind::Index => 40.0,
            ColumnKind::Name => 200.0,
            ColumnKind::Department => 200.0,
            ColumnKind::Edit => 100.0,
        };

        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

struct Row {
    name: String,
    department: String,
}

impl<'a> table::Column<'a, ListUsersMessage, Theme, Renderer> for Column {
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, ListUsersMessage> {
        let content = match self.kind {
            ColumnKind::Name => "Name",
            ColumnKind::Department => "Department",
            ColumnKind::Edit => "",
        };

        container(text(content)).center_y(24).into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a Row,
    ) -> Element<'a, ListUsersMessage> {
        let content: Element<_> = match self.kind {
            ColumnKind::Name => Text::new(format!("{}", row.name)).into(),
            ColumnKind::Department => Text::new(format!("{}", row.department)).into(),

            ColumnKind::Edit => button(text("Edit"))
                .on_press(ListUsersMessage::Edit(row_index))
                .into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn footer(
        &'a self,
        _col_index: usize,
        rows: &'a [Row],
    ) -> Option<Element<'a, ListUsersMessage>> {
        let content = {
            let total_enabled = rows.len();

            Element::from(text(format!("Total Enabled: {total_enabled}")))
        };
        Some(container(content).center_y(24).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
