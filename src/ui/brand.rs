use iced::widget::combo_box::State;
use iced::widget::{button, column, combo_box};
use iced::{Element, Right, Task};
use iced_aw::sidebar::TabLabel;
use reqwest;

use crate::{Message, Tab, WelcomeElement};

#[derive(Debug, Default)]
pub struct TestAsyncTab {
    text_name: String,
    funcs: State<String>,
}

#[derive(Debug, Clone)]
pub enum TestAsyncMessage {
    Search,
    Update(Vec<WelcomeElement>),

    NameChanged(String),
}
pub enum TestAsyncAction {
    None,
    Run(Task<TestAsyncMessage>),
}

async fn get_data() -> Vec<WelcomeElement> {
    reqwest::get("https://jsonplaceholder.typicode.com/users")
        .await
        .unwrap()
        .json::<Vec<WelcomeElement>>()
        .await
        .unwrap()
}
impl TestAsyncTab {
    pub fn new() -> (Self, Task<TestAsyncMessage>) {
        (
            Self {
                text_name: String::new(),
                funcs: State::new(Vec::new()),
                // combox_box_name: String::new(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: TestAsyncMessage) -> TestAsyncAction {
        dbg!(&message);
        match message {
            TestAsyncMessage::Update(value) => {
                let tmp = value.iter().map(|f| f.name.clone()).collect();
                self.funcs = State::new(tmp);

                TestAsyncAction::None
            }
            TestAsyncMessage::Search => {
                let tmp = Task::perform(get_data(), TestAsyncMessage::Update);
                TestAsyncAction::Run(tmp)
            }
            TestAsyncMessage::NameChanged(value) => {
                self.text_name = value;
                TestAsyncAction::None
            }
        }
    }
}

impl Tab for TestAsyncTab {
    type Message = Message;

    fn title(&self) -> String {
        self.text_name.clone()
    }

    fn tab_label(&self) -> TabLabel {
        TabLabel::Text("async combo_box".to_string())
    }
    fn content(&self) -> Element<'_, Message> {
        let content: Element<TestAsyncMessage> = column![
            button("search names").on_press(TestAsyncMessage::Search),
            combo_box(
                &self.funcs,
                "change one name",
                Some(&self.text_name),
                TestAsyncMessage::NameChanged
            )
        ]
        .max_width(500)
        .spacing(20)
        .align_x(Right)
        .into();
        content.map(Message::AsyncTest)
    }
}
