use iced::{
    Element,
    Length::Fill,
    widget::{button, column, container, row, text},
};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        container(
            column![
                button("+").on_press(Message::Increment),
                row!["Counter:", text(self.value).size(20)].spacing(10),
                button("-").on_press(Message::Decrement),
            ]
            .spacing(10),
        )
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    iced::run("A cool counter", Counter::update, Counter::view)?;

    Ok(())
}
