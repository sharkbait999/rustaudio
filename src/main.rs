use iced::{button, futures::Sink, Button, Column, Element, Sandbox, Settings, Text};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    file: Option<PathBuf>, // Maybe move these to it's own Player state?
    sink: Option<Sink>,
    load_button_state: button::State,
    play_button_state: button::State,
    pause_button_state: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    LoadPressed,
    PlayPressed,
    PausePressed,
}

fn load_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("mp3, flac, wav", &["mp3", "flac", "wav"])
        .pick_file()
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App {
            file: None,
            sink: None,
            load_button_state: button::State::new(),
            play_button_state: button::State::new(),
            pause_button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Audio Player")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LoadPressed => {
                self.file = load_file();
            }
            Message::PlayPressed => {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                self.sink = Sink::try_new(&stream_handle).unwrap();

                let file = BufReader::new(File::open(self.file.as_ref().unwrap()).unwrap());
                // Decode that sound file into a source
                let source = Decoder::new(file).unwrap();
                // // Play the sound directly on the device

                let res = stream_handle.play_raw(source.convert_samples());
                std::thread::sleep(std::time::Duration::from_secs(3));
                if res.is_err() {
                    println!("FUCK!")
                }
            }
            Message::PausePressed => { /* Handle pause */ }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Button::new(&mut self.load_button_state, Text::new("Load"))
                    .on_press(Message::LoadPressed),
            )
            .push(
                Button::new(&mut self.play_button_state, Text::new("Play"))
                    .on_press(Message::PlayPressed),
            )
            .push(
                Button::new(&mut self.pause_button_state, Text::new("Pause"))
                    .on_press(Message::PausePressed),
            )
            .into()
    }
}
