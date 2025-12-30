use std::collections::HashMap;

use cosmic::iced::{self, Alignment, Background, Color, Length};
use cosmic::widget::{
    self as widget, button, column, container, row, scrollable, table, text_input,
};
use cosmic::{Action, Apply, Element};

use crate::database;
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum BrandsMessage {
    // LoadedBrands(Vec<database::model::DbBrand>),
    GetBrands(Vec<database::model::DbBrand>),
    CreateBrand,
    ChangingName(String),
    BrandCreated(bool),
    DeleteBrand,
    BrandDeleted(bool),
    ItemSelect(table::Entity),
    OpenCreateModal,
    CloseCreateModal,
}

pub struct BrandsTab {
    brands: table::SingleSelectModel<Item, Category>,
    brand: String,
    show_create_modal: bool,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
enum Category {
    #[default]
    Name,
}

#[derive(Default, Debug, Clone)]
struct Item {
    name: String,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Name => "Name",
        })
    }
}

impl table::ItemCategory for Category {
    fn width(&self) -> iced::Length {
        match self {
            Self::Name => iced::Length::Fixed(300.0),
        }
    }
}

impl table::ItemInterface<Category> for Item {
    fn get_icon(&self, _category: Category) -> Option<cosmic::widget::Icon> {
        None
    }

    fn get_text(&self, category: Category) -> std::borrow::Cow<'static, str> {
        match category {
            Category::Name => self.name.clone().into(),
        }
    }

    fn compare(&self, other: &Self, category: Category) -> std::cmp::Ordering {
        match category {
            Category::Name => self.name.to_lowercase().cmp(&other.name.to_lowercase()),
        }
    }
}

impl BrandsTab {
    pub fn new() -> (Self, Task<BrandsMessage>) {
        let table_model = table::Model::new(vec![Category::Name]);
        let app = Self {
            brands: table_model,
            brand: String::new(),

            show_create_modal: false,
        };
        let command = Task::perform(database::get_brands(), |arg| {
            let tmp = arg.unwrap_or_default();
            Action::App(BrandsMessage::GetBrands(tmp))
        });
        (app, command)
    }

    pub fn update(&mut self, message: BrandsMessage) -> Task<BrandsMessage> {
        match message {
            BrandsMessage::GetBrands(brands) => {
                let mut table_mode = table::Model::new(vec![]);

                brands.into_iter().for_each(|i| {
                    let tmp = Item { name: i.name };
                    let _ = table_mode.insert(tmp);
                });
                self.brands = table_mode;
            }
            BrandsMessage::ItemSelect(entity) => self.brands.activate(entity),

            BrandsMessage::CreateBrand => {
                let command = Task::perform(database::insert_brand(self.brand.clone()), |arg| {
                    let result = match arg {
                        Ok(_) => true,
                        Err(_) => false,
                    };
                    Action::App(BrandsMessage::BrandCreated(result))
                });
                return command;
            }
            BrandsMessage::DeleteBrand => {
                let brand = self
                    .brands
                    .item(self.brands.active())
                    .cloned()
                    .unwrap_or_default()
                    .name;

                let command = Task::perform(database::delete_brand(brand.clone()), |arg| {
                    let result = match arg {
                        Ok(_) => true,
                        Err(_) => false,
                    };
                    Action::App(BrandsMessage::BrandDeleted(result))
                });
                return command;
            }
            BrandsMessage::BrandDeleted(deleteado) => {
                if deleteado {
                    let command = Task::perform(database::get_brands(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(BrandsMessage::GetBrands(tmp))
                    });
                    return command;
                }
            }
            BrandsMessage::BrandCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_brands(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(BrandsMessage::GetBrands(tmp))
                    });
                    self.show_create_modal = false;
                    return command;
                }
            }
            BrandsMessage::ChangingName(name) => {
                self.brand = name;
            }

            BrandsMessage::OpenCreateModal => {
                self.show_create_modal = true;
            }
            BrandsMessage::CloseCreateModal => {
                self.show_create_modal = false;
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, BrandsMessage> {
        let brand_delete = button::text("criar marca").on_press(BrandsMessage::OpenCreateModal);

        let create_brand: Element<'_, BrandsMessage> =
            row().push(brand_delete).align_y(Alignment::Center).into();

        let brand_list = widget::compact_table(&self.brands)
            .on_item_left_click(BrandsMessage::ItemSelect)
            .item_context(|item| {
                Some(widget::menu::items(
                    &HashMap::new(),
                    vec![widget::menu::Item::Button(
                        format!("Excluir Brand on {}", item.name),
                        None,
                        MyAction::Selecionado,
                    )],
                ))
            })
            .on_item_right_click(BrandsMessage::ItemSelect)
            .apply(Element::from);

        let scrol = scrollable(brand_list);

        let content = column()
            .push(create_brand)
            .push(column().push(scrol).spacing(8))
            .spacing(16)
            .padding(20)
            .max_width(600);

        let base = container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        if self.show_create_modal {
            let brand_input =
                text_input("Brand name", &self.brand).on_input(BrandsMessage::ChangingName);

            let actions = row()
                .push(button::text("Cancel").on_press(BrandsMessage::CloseCreateModal))
                .push(button::text("Create").on_press(BrandsMessage::CreateBrand))
                .spacing(8);

            let modal_content = container(
                column()
                    .push(brand_input)
                    .push(actions)
                    .spacing(12)
                    .padding(20)
                    .width(Length::Fixed(400.0)),
            )
            .style(|theme: &cosmic::Theme| {
                let cosmic = theme.cosmic();
                iced::widget::container::Style {
                    background: Some(Background::Color(Color::from(cosmic.primary.base))),
                    text_color: Some(Color::from(cosmic.primary.on)),
                    ..Default::default()
                }
            });

            let tmp = widget::popover(base)
                .modal(true)
                .position(widget::popover::Position::Center)
                .on_close(BrandsMessage::CloseCreateModal);

            return tmp.popup(modal_content).into();
        }

        base.into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MyAction {
    Selecionado,
}

impl widget::menu::Action for MyAction {
    type Message = BrandsMessage;

    fn message(&self) -> Self::Message {
        BrandsMessage::DeleteBrand
    }
}
