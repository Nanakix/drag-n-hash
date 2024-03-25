use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

// use iced_drop::droppable;
struct Hashword {
    value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GetValuePressed,
    ClearValuePressed,
}

impl Sandbox for Hashword {
    type Message = Message;

    fn new() -> Self {
        Self {value: String::from("base") }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("get").on_press(Message::GetValuePressed),
            text(&self.value.clone()).size(30),
            button("clear").on_press(Message::ClearValuePressed),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::GetValuePressed => {
                let mut m = sha1_smol::Sha1::new();
                m.update(b"Hello World!");
                self.value = m.digest().to_string()
            },
            Message::ClearValuePressed => {
                self.value = "".into();
            }
        }
    }

    fn title(&self) -> String {
        String::from("Hashword")
    }
}


pub fn main() -> iced::Result {
    Hashword::run(Settings::default())
}

