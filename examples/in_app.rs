use iced::Application as _;

const LOADING_INDICATOR_SIZE: f32 = 200.0;
const LOADING_INDICATOR_SPEED_MS: u64 = 85;

fn main() -> iced::Result {
    MyApp::run(iced::Settings {
        window: iced::window::Settings {
            size: (400, 300),
            position: iced::window::Position::Specific(50, 800),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct MyApp {
    index: iced_loading_indicator::Index,
}

#[derive(Debug, Clone)]
enum MyMessage {
    Tick,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            index: Default::default(),
        }
    }
}

impl iced::Application for MyApp {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = MyMessage;
    type Theme = iced::theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Default::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        "Loading Indicator Demo".into()
    }

    fn theme(&self) -> Self::Theme {
        iced::theme::Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(LOADING_INDICATOR_SPEED_MS))
            .map(|_| MyMessage::Tick)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let canvas =
            iced_loading_indicator::LoadingIndicator::new(LOADING_INDICATOR_SIZE, self.index)
                .style(iced_loading_indicator::Style::PrimaryColor);

        iced::widget::container(
            iced::widget::column!(canvas)
                .align_items(iced::Alignment::Center)
                .spacing(20),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .padding(0)
        .into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyMessage::Tick => {
                self.index.tick();
                iced::Command::none()
            }
        }
    }
}
