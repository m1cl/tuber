mod dropdown;
mod style;
mod youtube;

use dropdown::DropDownItems;
use iced::{
    executor, image, pick_list, scrollable, text_input, Align, Application, Column, Command,
    Container, Element, Image, Length, PickList, Radio, Scrollable, Settings, Text, TextInput,
};
static HEIGHT: u32 = 800;
use tokio::runtime::Runtime;

// use yup_oauth2::AccessToken;

type UserName = String;

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    InputChanged(String),
    InputSubmit(UserName),
    ThemeChanged(style::sheet::Theme),
    LanguageSlected(DropDownItems),
}

struct PlaylistItems {
    item: String,
}
#[derive(Default, Debug)]
struct Gui {
    theme: style::sheet::Theme,
    value: i32,
    scroll: scrollable::State,
    input_value: String,
    pick_list: pick_list::State<DropDownItems>,
    input: text_input::State,
    selected_language: DropDownItems,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut rt = Runtime::new().unwrap();
        let res = rt.block_on(youtube::Youtube::get_playlists()).unwrap();
        (
            // initialState
            Gui {
                ..Default::default()
            },
            Command::none(),
        )
        // Self::default()
    }

    fn view(&mut self) -> Element<Message> {
        // We use a column: a simple vertical layout
        let Gui { scroll, .. } = self;
        // let choose_theme = style::sheet::Theme::ALL.iter().fold(
        // Column::new().spacing(10).push(Text::new("Choose theme")),
        // |column, theme| {
        //     column.push(
        //         Radio::new(
        //             *theme,
        //             &format!("{:?}", theme),
        //             Some(theme),
        //             Message::ThemeChanged,
        //         )
        //         .style(self.theme),
        //     )
        // },
        // );

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
        let wy = Image::new("./wysiwyg.png");
        let image_column = Column::new().push(wy);
        // let image  = Image::new("./wysiwyg.png");

        // let playlists = PickList::new(
        //     &mut self.pick_list,
        //     &DropDownItems::ALL[..],
        //     Some(self.selected_language),
        //     Message::LanguageSlected,
        // );
        let playlist_item = Text::new("Rock");
        let playlist_item2 = Text::new("Pop");
        let playlist_item3 = Text::new("Live");
        let playlist_item4 = Text::new("Indie");
        let playlist_item5 = Text::new("dark wave");
        let playlist_item6 = Text::new("new wave");
        let playlist_item7 = Text::new("post punk");
        let playlist_item8 = Text::new("black metal");
        let playlist_item9 = Text::new("depressive black metal");
        let playlist_item10 = Text::new("tekkno");
        // let col = Column::new().max_width(800).spacing(20).push(stuff);
        let scroll_container: Scrollable<Message> = Scrollable::new(scroll)
            .push(playlist_item)
            .push(playlist_item2)
            .push(playlist_item3)
            .push(playlist_item4)
            .push(playlist_item5)
            .push(playlist_item6)
            .push(playlist_item7)
            .push(playlist_item8)
            .push(playlist_item9)
            .push(playlist_item10);

        let playlists_container = Column::new()
            .max_width(300)
            .spacing(20)
            .max_height(130)
            .push(scroll_container);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .max_width(HEIGHT)
            .align_items(Align::End)
            // .push(svg_container)
            // .push(choose_theme)
            // .push(playlists)
            .push(wy)
            .push(playlists_container)
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
            Message::LanguageSlected(language) => self.selected_language = language,
            _ => {}
        }
        Command::none()
    }
}

fn main() -> iced::Result {
    Gui::run(Settings::default())
}
