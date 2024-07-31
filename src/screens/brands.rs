use iced::advanced::graphics::text::cosmic_text::Align;
use iced::widget::button;
use iced::widget::text_input;
// use iced::widget::Align;
use iced::widget::{Button, Column, Row, Text, TextInput};
use iced::Theme;
use iced::{Application, Command, Element, Settings};

#[derive(Debug, Clone)]
pub enum Message {
    TextInputChanged(String),
    AddPressed,
    DeletePressed,
    ListItemSelected(usize),
}

pub struct GlobalBrand {
    items: Vec<String>,
    current: Option<usize>,
    text: String,
    add_button: button::State,
    delete_button: button::State,
    input_state: String,
}

impl Application for GlobalBrand {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            GlobalBrand {
                items: Vec::new(),
                current: None,
                text: String::new(),
                add_button: button::State::new(),
                delete_button: button::State::new(),
                input_state: String::new(), //text_input::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Brand Manager")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextInputChanged(new_text) => {
                self.text = new_text;
            }
            Message::AddPressed => {
                if !self.text.is_empty() {
                    self.items.push(self.text.clone());
                    self.text.clear();
                }
            }
            Message::DeletePressed => {
                if let Some(current) = self.current {
                    self.items.remove(current);
                    self.current = None;
                }
            }
            Message::ListItemSelected(index) => {
                self.current = Some(index);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let input = TextInput::new("Enter text...", &self.text)
            .padding(10)
            .size(20);

        let add_button = Button::new(Text::new("Add")).on_press(Message::AddPressed);

        let delete_button = Button::new(Text::new("Delete")).on_press(Message::DeletePressed);

        // &mut self.delete_button,

        let mut list = Column::new().spacing(10).padding(10);

        for (index, item) in self.items.iter().enumerate() {
            let item_button =
                Button::new(Text::new(item)).on_press(Message::ListItemSelected(index));

            list = list.push(item_button);
        }

        let controls = Row::new()
            .spacing(10)
            .push(input)
            .push(add_button)
            .push(delete_button);

        Column::new()
            .spacing(20)
            // .align_items(Align::Center)
            .push(controls)
            .push(list)
            .into()
    }
}

fn main() -> iced::Result {
    GlobalBrand::run(Settings::default())
}
