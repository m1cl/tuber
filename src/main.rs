use iced::{
    executor, text_input, Application, Column, Command, Container, Element, Length, Radio,
    Settings, Text, TextInput,
};
use tokio::runtime::Runtime;

// use yup_oauth2::AccessToken;
mod style;
mod youtube;

type UserName = String;

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    InputChanged(String),
    InputSubmit(UserName),
    ThemeChanged(style::sheet::Theme),
}

// type Json = HashMap<String, String>;

struct Gui {
    theme: style::sheet::Theme,
    value: i32,
    input_value: String,
    input: text_input::State,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(youtube::Youtube::auth());
        // TODO: It should panick if the clientsecret is not right
        println!("thre result {:?}", result);

        (
            // initialState
            Gui {
                theme: style::sheet::Theme::default(),
                value: 0,
                input: text_input::State::default(),
                input_value: format!(""),
            },
            Command::none(),
        )
        // Self::default()
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout

        let choose_theme = style::sheet::Theme::ALL.iter().fold(
            Column::new().spacing(10).push(Text::new("Choose theme")),
            |column, theme| {
                column.push(
                    Radio::new(
                        *theme,
                        &format!("{:?}", theme),
                        Some(self.theme),
                        Message::ThemeChanged,
                    )
                    .style(self.theme),
                )
            },
        );

        // let svg = Svg::from_path(format!("../assets/background.svg"))
        //     .width(Length::Fill)
        //     .height(Length::Fill);

        // let svg_container = Column::new().spacing(20).push(svg);

        // let query = "https://api.discogs.com/releases/249504".to_string();

        let text_input = TextInput::new(
            // get current state of input
            &mut self.input,
            "youtube user name",
            &self.input_value,
            Message::InputChanged,
        )
        .size(18)
        .padding(30)
        .style(self.theme);

        let user_name = self.input_value.to_string();

        // TODO: try to use a pointer, data waste
        let text_input = text_input.on_submit(Message::InputSubmit(user_name));

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(800)
            // .push(svg_container)
            .push(choose_theme)
            .push(text_input);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()
    }

    fn title(&self) -> String {
        String::from("this is the title")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }

            Message::InputSubmit(user_name) => {}

            Message::InputChanged(value) => self.input_value = value,
            Message::ThemeChanged(theme) => self.theme = theme,
        }
        Command::none()
    }
}
fn main() -> Result<(), iced::Error> {
    Gui::run(Settings::default())
}
