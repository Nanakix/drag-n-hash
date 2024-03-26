use iced::widget::{button, column, text};
use iced::{Alignment, Application, Element, Length, Sandbox, Settings, theme};
use iced::widget::container;
use iced::widget::container::Id as CId;
use iced_drop::droppable;
struct Hashword {
    filename: String,
    sha1:  String,
    up: iced::widget::container::Id,
    down: iced::widget::container::Id,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GetValue,
    ClearValue,
}

impl Default for Hashword {
    fn default() -> Self {
        Self {
            filename: "Drag and drop a file here".to_string(),
            sha1: "deadbeefcafe".to_string(),
            up: CId::new("up"),
            down: CId::new("down"),
        }
    }
}

impl Application for Hashword {
    type Executor = iced::executor::Default;
    type Message = Message;

    type Theme = iced::theme::Theme;

    type Flags = ();
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), iced::Command::none())
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        column![
            button("get").on_press(Message::GetValue),
            text(&self.filename.clone()).size(30),
            button("clear").on_press(Message::ClearValue),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) -> iced::Command<Self::Message> {
        match message {
            Message::GetValue => {
                let mut m = sha1_smol::Sha1::new();
                m.update(b"Hello World!");
                self.filename = m.digest().to_string()
            },
            Message::ClearValue => {
                self.filename = "".into();
                self.sha1 = "".into();
            }
        }
        iced::Command::none()
    }

    fn title(&self) -> String {
        String::from("Drag-n-hash")
    }
}


pub fn main() -> iced::Result {
    Hashword::run(Settings::default())
}

