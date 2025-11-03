use cosmic::{
    Action, Element,
    iced::{
        Alignment, Length,
        widget::{column, row},
    },
    iced_widget::text_input,
    widget::{container, text},
};

use crate::database;
use cosmic::app::Task;

#[derive(Debug, Clone)]
pub enum BrandsMessage {
    GetBrands(Vec<database::model::DbBrand>),
    CreateBrand,
    ChangingName(String),
    BrandCreated(bool),
}

#[derive(Default)]
pub struct BrandsTab {
    brands: Vec<database::model::DbBrand>,
    brand: String,
}

impl BrandsTab {
    pub fn new() -> (Self, Task<BrandsMessage>) {
        let app = Self {
            brands: Vec::new(),
            brand: String::new(),
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
                self.brands = brands;
                Task::none()
            }
            BrandsMessage::CreateBrand => {
                let command = Task::perform(database::insert_brand(self.brand.clone()), |arg| {
                    let result = match arg {
                        Ok(_) => true,
                        Err(_) => false,
                    };
                    Action::App(BrandsMessage::BrandCreated(result))
                });
                command
            }
            BrandsMessage::BrandCreated(criado) => {
                if criado {
                    let command = Task::perform(database::get_brands(), |arg| {
                        let tmp = arg.unwrap_or_default();
                        Action::App(BrandsMessage::GetBrands(tmp))
                    });
                    return command;
                }

                return Task::none();
            }
            BrandsMessage::ChangingName(name) => {
                self.brand = name;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, BrandsMessage> {
        let brand_text = text_input("Nova Marca", &self.brand)
            .on_input(BrandsMessage::ChangingName)
            .on_submit(BrandsMessage::CreateBrand);
        let create_brand: Element<'_, BrandsMessage> = row![brand_text].into();
        let brand_list: Vec<Element<'_, BrandsMessage>> = self
            .brands
            .iter()
            .map(|brand| Element::from(text(format!("marca:{}", brand.name))))
            .collect();
        let content = column![create_brand]
            .push(column(brand_list).spacing(8))
            .spacing(16)
            .padding(20)
            .max_width(600);

        container(content)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }
}
