#![allow(dead_code)]
use iced::widget::{button, text, Column, Text, TextInput};
use iced::{executor, Application, Command, Element, Settings};
#[derive(Default)]
struct PlotTracker {
    characters: Vec<String>,
    marks: Vec<String>,
    new_mark: String,
}
#[derive(Debug, Clone)]
enum Message {
    AddCharacter(String),
    AddMark(String),
    PushNewCharacter,
    PushNewMark,
}
impl Application for PlotTracker {
    type Flags = ();
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            PlotTracker {
                characters: vec![],
                marks: vec![],
                new_mark: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Plot Tracker")
    }
    fn update(&mut self, message: Message) -> Command<self::Message> {
        match message {
            Message::AddCharacter(new_character) => {
                if !new_character.is_empty() {
                    self.characters.push(new_character);
                }
            }
            Message::AddMark(new_mark) => {
                if !new_mark.is_empty() {
                    self.marks.push(new_mark);
                }
            }
            Message::PushNewMark => {
                todo!();
            }
            Message::PushNewCharacter => {
                todo!();
            }
        }
        Command::none()
    }
    fn view(&self) -> Element<Message> {
        let m = self.marks.iter().fold(Column::new(), |column, mark| {
            column.push(Text::new(mark.clone()))
        });
        let c = self
            .characters
            .iter()
            .fold(Column::new(), |column, characters| {
                column.push(Text::new(characters.clone()))
            });
        Column::new()
            .push(text("Marks:"))
            .push(m)
            .push(TextInput::new("new_mark", &self.new_mark))
            .push(text("Characters: "))
            .push(c)
            .into()
    }
}
fn main() -> iced::Result {
    PlotTracker::run(Settings::default())
}
