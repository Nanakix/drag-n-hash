use std::fs::File;
use std::io::Read;
use humansize::{FormatSize, BINARY};
use iced::alignment;
use iced::clipboard;
use iced::event::{self, Event};
use iced::executor;
use iced::widget::{button, container, text, Column};
use iced::window;
use iced::{
    Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};
use md5::Md5;
use sha1_smol::Sha1;
use sha2::{Digest, Sha256};


pub fn main() -> iced::Result {
    Events::run(Settings {
        window: window::Settings {
            exit_on_close_request: false,
            ..window::Settings::default()
        },

        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct Events {
    rom_name: String,
    md5: String,
    sha1: String,
    sha256: String,
    crc32: String,
    rom_size: String,
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Exit,
}
impl Application for Events {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        (Events::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Drag-n-hash")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let Event::Window(id, window::Event::CloseRequested) = event
                {
                   return window::close(id);
                }
                if let Event::Window(_id, window::Event::FileDropped(path)) = event
                {
                    let mut f = File::open(path.clone()).unwrap();
                    let mut buffer = Vec::new();
                    f.read_to_end(&mut buffer).unwrap();

                    self.rom_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    self.crc32 = format!("CRC32: {:X}", crc32fast::hash(buffer.as_slice()));
                    self.sha1 = format!("SHA1: {}", Sha1::from(buffer.clone()).hexdigest().to_uppercase());
                    self.sha256 = format!("SHA256: {:X}", Sha256::digest(buffer.clone()));
                    self.md5 = format!("MD5: {:X}", Md5::digest(buffer.clone()));
                    self.rom_size = format!("Size: {} bytes, ({})", buffer.len(), buffer.len().format_size(BINARY));
                    clipboard::write(format!("{}\n{}\n{}\n{}\n{}\n{}\n",self.rom_name, self.crc32, self.sha1, self.sha256, self.md5, self.rom_size))
                }
                else {
                    Command::none()
                }
            }
            Message::Exit => window::close(window::Id::MAIN),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn view(&self) -> Element<Message> {
        let dropped = Column::with_children(
            [self.rom_name.as_str(),
                 self.md5.as_str(),
                 self.sha1.as_str(),
                 self.sha256.as_str(),
                 self.crc32.as_str(),
                 self.rom_size.as_str()]
                .iter()
                .map(|event| text(event.to_string()).size(20))
                .map(Element::from),
        );


        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
            .width(100)
            .padding(10)
            .on_press(Message::Exit);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(dropped)
            .push(exit);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}