use std::io::Read;
use flume::{Receiver, Sender};
use humansize::{FormatSize, BINARY};
use iced::{alignment, time};
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
use drag_n_hash::{Status, Toast};


pub fn main() -> iced::Result {
    Events::run(Settings {
        window: window::Settings {
            exit_on_close_request: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Events {
    rom_name: String,
    md5: String,
    sha1: String,
    sha256: String,
    crc32: String,
    rom_size: String,
    md5_send: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    sha1_send: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    sha256_send: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    crc32_send: (Sender<Vec<u8>>, Receiver<Vec<u8>>),
    md5_receive: (Sender<String>, Receiver<String>),
    sha1_receive: (Sender<String>, Receiver<String>),
    sha256_receive: (Sender<String>, Receiver<String>),
    crc32_receive: (Sender<String>, Receiver<String>),
    running: bool,
    toasts: Vec<Toast>,
    finished: bool,
}

impl Default for Events {
    fn default() -> Self {
        let md5_send = flume::unbounded();
        let sha1_send = flume::unbounded();
        let sha256_send = flume::unbounded();
        let crc32_send = flume::unbounded();
        let md5_receive = flume::unbounded();
        let sha1_receive = flume::unbounded();
        let sha256_receive = flume::unbounded();
        let crc32_receive = flume::unbounded();

        Self {
            rom_name: "".to_string(),
            md5: "".to_string(),
            sha1: "".to_string(),
            sha256: "".to_string(),
            crc32: "".to_string(),
            rom_size: "".to_string(),
            md5_send,
            sha1_send,
            sha256_send,
            crc32_send,
            md5_receive,
            sha1_receive,
            sha256_receive,
            crc32_receive,
            running: false,
            toasts: vec![],
            finished: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Check,
    CopyToClipboard,
    Exit,
}
impl Application for Events {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Events, Command<Message>) {
        let events = Events::default();
        (events, Command::none())
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
                    if self.running {
                        return Command::none();
                    }
                    self.running = true;
                    self.finished = false;

                    let md5_rx = self.md5_send.1.clone();
                    let md5_tx = self.md5_receive.0.clone();
                    std::thread::spawn(move || {
                        let mut md5 = Md5::new();
                        while let Ok(chunk) = md5_rx.recv() {
                            if chunk.is_empty() {
                                break;
                            }
                            md5.update(&chunk);
                        }
                        let _ = md5_tx.send(format!("MD5: {:X}", md5.finalize()));
                    });

                    let sha1_rx = self.sha1_send.1.clone();
                    let sha1_tx = self.sha1_receive.0.clone();
                    std::thread::spawn(move || {
                        let mut sha1 = Sha1::new();
                        while let Ok(chunk) = sha1_rx.recv() {
                            if chunk.is_empty() {
                                break;
                            }
                            sha1.update(&chunk);
                        }
                        let _ = sha1_tx.send(format!("SHA1: {}", sha1.hexdigest().to_uppercase()));
                    });

                    let sha256_rx = self.sha256_send.1.clone();
                    let sha256_tx = self.sha256_receive.0.clone();
                    std::thread::spawn(move || {
                        let mut sha256 = Sha256::new();
                        while let Ok(chunk) = sha256_rx.recv() {
                            if chunk.is_empty() {
                                break;
                            }
                            sha256.update(&chunk);
                        }
                        let _ = sha256_tx.send(format!("SHA256: {:X}", sha256.finalize()));
                    });

                    let crc32_rx = self.crc32_send.1.clone();
                    let crc32_tx = self.crc32_receive.0.clone();
                    std::thread::spawn(move || {
                        let mut crc32 = crc32fast::Hasher::new();
                        while let Ok(chunk) = crc32_rx.recv() {
                            if chunk.is_empty() {
                                break;
                            }
                            crc32.update(&chunk);
                        }
                        let _ = crc32_tx.send(format!("CRC32: {:X}", crc32.finalize()));
                    });

                    let mut f = std::fs::File::open(&path).unwrap();
                    let len = f.metadata().unwrap().len();

                    self.md5 = String::from("MD5: Computing...");
                    self.sha1 = String::from("SHA1: Computing...");
                    self.sha256 = String::from("SHA256: Computing...");
                    self.crc32 = String::from("CRC32: Computing...");
                    self.rom_name = path.file_name().unwrap().to_str().unwrap().to_string();
                    self.rom_size = format!("Size: {} bytes, ({})", len, len.format_size(BINARY));

                    let md5_send_cloned = self.md5_send.0.clone();
                    let sha1_send_cloned = self.sha1_send.0.clone();
                    let sha256_send_cloned = self.sha256_send.0.clone();
                    let crc32_send_cloned = self.crc32_send.0.clone();

                    std::thread::spawn(move || {
                        let mut chunk = vec![0; 500_000];
                        while let Ok(read) = f.read(&mut chunk) {
                            if read == 0 {
                                md5_send_cloned.send(Vec::default()).unwrap();
                                sha1_send_cloned.send(Vec::default()).unwrap();
                                sha256_send_cloned.send(Vec::default()).unwrap();
                                crc32_send_cloned.send(Vec::default()).unwrap();
                                break;
                            }
                            let chunk = chunk[..read].to_vec();
                            md5_send_cloned.send(chunk.clone()).unwrap();
                            sha1_send_cloned.send(chunk.clone()).unwrap();
                            sha256_send_cloned.send(chunk.clone()).unwrap();
                            crc32_send_cloned.send(chunk.clone()).unwrap();
                        }
                    });

                    Command::none()
                }
                else {
                    Command::none()
                }
            }
            Message::Check => {
                if self.running {
                    if self.md5 == "MD5: Computing..." {
                        self.md5 = self.md5_receive.1.try_recv().unwrap_or_else(|_| String::from("MD5: Computing..."));
                    }

                    if self.sha1 == "SHA1: Computing..." {
                        self.sha1 = self.sha1_receive.1.try_recv().unwrap_or_else(|_| String::from("SHA1: Computing..."));
                    }

                    if self.sha256 == "SHA256: Computing..." {
                        self.sha256 = self.sha256_receive.1.try_recv().unwrap_or_else(|_| String::from("SHA256: Computing..."));
                    }

                    if self.crc32 == "CRC32: Computing..." {
                        self.crc32 = self.crc32_receive.1.try_recv().unwrap_or_else(|_| String::from("CRC32: Computing..."));
                    }
                }

                if self.md5 != "MD5: Computing..." && self.sha1 != "SHA1: Computing..." && self.sha256 != "SHA256: Computing..." && self.crc32 != "CRC32: Computing..." {
                    self.running = false;
                    self.toasts = vec![Toast {
                        title: "Done".into(),
                        body: "Computation finished, result has been written to clipboard".into(),
                        status: Status::Primary,
                    }];
                    if !self.finished {
                        self.finished = true;
                        return clipboard::write(format!("{}\n{}\n{}\n{}\n{}\n{}\n",self.rom_name, self.crc32, self.sha1, self.sha256, self.md5, self.rom_size));
                    }
                }
                Command::none()
            }
            Message::CopyToClipboard => clipboard::write(format!("{}\n{}\n{}\n{}\n{}\n{}\n",self.rom_name, self.crc32, self.sha1, self.sha256, self.md5, self.rom_size)),
            Message::Exit => window::close(window::Id::MAIN),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([event::listen().map(Message::EventOccurred), time::every(time::Duration::from_millis(100)).map(|_| Message::Check)])
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

        let _copy_again = button(
            text("Copy to clipboard")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
            .width(180)
            .padding(10)
            .on_press(Message::CopyToClipboard);

        let exit = button(
            text("Exit")
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
            .width(180)
            .padding(10)
            .on_press(Message::Exit);

        let content = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(dropped)
            .push(_copy_again)
            .push(exit);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}