use iced::Application as _;

const SPACING_SMALL: u16 = 5;
const SPACING_NORMAL: u16 = 10;
const SPACING_LARGE: u16 = 20;
const SPACING_LARGER: u16 = 30;

const INDICATOR_SIZE_MIN: f32 = 20.0;
const INDICATOR_SIZE_DEFAULT: f32 = 190.0;
const INDICATOR_SIZE_MAX: f32 = 350.0;

const INDICATOR_CUSTOM_SPEED_DEFAULT: u64 = 150;

const INDICATOR_CUSTOM_COLOR_DEFAULT_R: u8 = 0xff;
const INDICATOR_CUSTOM_COLOR_DEFAULT_G: u8 = 0xaa;
const INDICATOR_CUSTOM_COLOR_DEFAULT_B: u8 = 0x11;

const SIDEBAR_WIDTH: f32 = 300.0;

const INPUT_ID_CUSTOM_COLOR_R: &'static str = "custom_color_input_r";
const INPUT_ID_CUSTOM_SPEED: &'static str = "custom_speed_input";

fn main() -> iced::Result {
    MyApp::run(iced::Settings {
        window: iced::window::Settings {
            size: (800, 500),
            min_size: Some((800, 500)),
            position: iced::window::Position::Specific(50, 800),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct MyApp {
    dark_mode: bool,
    indicator_style: Option<IndicatorStyle>,
    indicator_speed: Option<IndicatorSpeed>,
    indicator_size: f32,
    indicator_custom_speed: u64,
    indicator_custom_color_r: u8,
    indicator_custom_color_g: u8,
    indicator_custom_color_b: u8,
}

#[derive(Debug, Clone)]
enum MyMessage {
    ResetButton,
    ThemeToggler(bool),
    IndicatorStylePicker(IndicatorStyle),
    IndicatorSpeedPicker(IndicatorSpeed),
    IndicatorSizeSlider(f32),
    IndicatorSpeedInput(u64),
    IndicatorColorInput(u8, Color),
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dark_mode: true,
            indicator_style: Some(IndicatorStyle::default()),
            indicator_speed: Some(IndicatorSpeed::default()),
            indicator_size: INDICATOR_SIZE_DEFAULT,
            indicator_custom_speed: INDICATOR_CUSTOM_SPEED_DEFAULT,
            indicator_custom_color_r: INDICATOR_CUSTOM_COLOR_DEFAULT_R,
            indicator_custom_color_g: INDICATOR_CUSTOM_COLOR_DEFAULT_G,
            indicator_custom_color_b: INDICATOR_CUSTOM_COLOR_DEFAULT_B,
        }
    }
}

impl MyApp {
    fn current_custom_color(&self) -> iced::Color {
        iced::Color::from_rgb8(
            self.indicator_custom_color_r,
            self.indicator_custom_color_g,
            self.indicator_custom_color_b,
        )
    }

    fn reset(&mut self) {
        self.dark_mode = true;
        self.indicator_style = Some(IndicatorStyle::default());
        self.indicator_speed = Some(IndicatorSpeed::default());
        self.indicator_size = INDICATOR_SIZE_DEFAULT;
        self.indicator_size = INDICATOR_SIZE_DEFAULT;
        self.indicator_custom_speed = INDICATOR_CUSTOM_SPEED_DEFAULT;
        self.indicator_custom_color_r = INDICATOR_CUSTOM_COLOR_DEFAULT_R;
        self.indicator_custom_color_g = INDICATOR_CUSTOM_COLOR_DEFAULT_G;
        self.indicator_custom_color_b = INDICATOR_CUSTOM_COLOR_DEFAULT_B;
    }

    fn view_sidebar(&self) -> iced::Element<'_, MyMessage> {
        let theme_toggler = {
            let label = iced::widget::text("Dark Mode:");

            let toggler = iced::widget::toggler(None, self.dark_mode, MyMessage::ThemeToggler)
                .width(iced::Length::Shrink)
                .text_alignment(iced::alignment::Horizontal::Center);

            let spacer = iced::widget::horizontal_space(iced::Length::Fill);

            iced::widget::row!(label, spacer, toggler)
                .spacing(SPACING_SMALL)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .align_items(iced::Alignment::Center)
        };

        let style_picker = {
            let label = iced::widget::text("Style:");

            let picker = iced::widget::pick_list(
                &IndicatorStyle::ALL[..],
                self.indicator_style,
                MyMessage::IndicatorStylePicker,
            );

            let spacer = iced::widget::horizontal_space(iced::Length::Fill);

            let row = iced::widget::row!(label, spacer, picker)
                .spacing(SPACING_SMALL)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .align_items(iced::Alignment::Center);

            let mut column = iced::widget::Column::new();
            column = column.push(row);

            if let Some(IndicatorStyle::CustomColor) = self.indicator_style {
                let input_r = {
                    let label = iced::widget::text("R:");

                    let input = numeric_input::NumericInput::new(
                        Some(self.indicator_custom_color_r),
                        |v| {
                            MyMessage::IndicatorColorInput(
                                v.unwrap_or(INDICATOR_CUSTOM_COLOR_DEFAULT_R),
                                Color::R,
                            )
                        },
                    )
                    .id(INPUT_ID_CUSTOM_COLOR_R);

                    iced::widget::row!(label, input)
                        .align_items(iced::Alignment::Center)
                        .spacing(SPACING_SMALL)
                };

                let input_g = {
                    let label = iced::widget::text("G:");

                    let input = numeric_input::NumericInput::new(
                        Some(self.indicator_custom_color_g),
                        |v| {
                            MyMessage::IndicatorColorInput(
                                v.unwrap_or(INDICATOR_CUSTOM_COLOR_DEFAULT_G),
                                Color::G,
                            )
                        },
                    );

                    iced::widget::row!(label, input)
                        .align_items(iced::Alignment::Center)
                        .spacing(SPACING_SMALL)
                };

                let input_b = {
                    let label = iced::widget::text("B:");

                    let input = numeric_input::NumericInput::new(
                        Some(self.indicator_custom_color_b),
                        |v| {
                            MyMessage::IndicatorColorInput(
                                v.unwrap_or(INDICATOR_CUSTOM_COLOR_DEFAULT_B),
                                Color::B,
                            )
                        },
                    );

                    iced::widget::row!(label, input)
                        .align_items(iced::Alignment::Center)
                        .spacing(SPACING_SMALL)
                };

                let spacer = iced::widget::horizontal_space(iced::Length::Fill);

                let inputs = iced::widget::row!(spacer, input_r, input_g, input_b)
                    .width(iced::Length::Fill)
                    .spacing(SPACING_NORMAL)
                    .align_items(iced::Alignment::Center);

                column = column.push(inputs);
            }

            column.spacing(SPACING_NORMAL)
        };

        let speed_picker = {
            let label = iced::widget::text("Speed:");

            let picker = iced::widget::pick_list(
                &IndicatorSpeed::ALL[..],
                self.indicator_speed,
                MyMessage::IndicatorSpeedPicker,
            );

            let spacer = iced::widget::horizontal_space(iced::Length::Fill);

            let row = iced::widget::row!(label, spacer, picker)
                .spacing(SPACING_SMALL)
                .width(iced::Length::Fill)
                .height(iced::Length::Shrink)
                .align_items(iced::Alignment::Center);

            let mut column = iced::widget::Column::new();
            column = column.push(row);

            if let Some(IndicatorSpeed::Custom) = self.indicator_speed {
                let input = {
                    let spacer = iced::widget::horizontal_space(iced::Length::Fill);

                    let label = iced::widget::text("Custom speed (ms):");

                    let input =
                        numeric_input::NumericInput::new(Some(self.indicator_custom_speed), |v| {
                            MyMessage::IndicatorSpeedInput(
                                v.unwrap_or(INDICATOR_CUSTOM_SPEED_DEFAULT),
                            )
                        })
                        .id(INPUT_ID_CUSTOM_SPEED);

                    iced::widget::row!(spacer, label, input)
                        .width(iced::Length::Fill)
                        .spacing(SPACING_NORMAL)
                        .align_items(iced::Alignment::Center)
                };

                column = column.push(input);
            }

            column.spacing(SPACING_NORMAL)
        };

        let reset_button = {
            let label = iced::widget::text("Reset")
                .width(iced::Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center);
            iced::widget::button(label)
                .width(iced::Length::Fill)
                .on_press(MyMessage::ResetButton)
        };

        let bottom_spacer = iced::widget::vertical_space(iced::Length::Fill);

        let column = iced::widget::column!(
            theme_toggler,
            style_picker,
            speed_picker,
            bottom_spacer,
            reset_button
        )
        .width(iced::Length::Shrink)
        .height(iced::Length::Fill)
        .align_items(iced::Alignment::Center)
        .spacing(SPACING_LARGER);

        iced::widget::container(column)
            .height(iced::Length::Fill)
            .width(iced::Length::Fixed(SIDEBAR_WIDTH))
            .padding(SPACING_LARGE)
            .into()
    }

    fn view_header_bar(&self) -> iced::Element<'_, MyMessage> {
        let size_slider = {
            let label = iced::widget::text("Size (px):");

            let min = iced::widget::text(format!("{}", INDICATOR_SIZE_MIN));

            let max = iced::widget::text(format!("{}", INDICATOR_SIZE_MAX));

            let slider = iced::widget::slider(
                INDICATOR_SIZE_MIN..=INDICATOR_SIZE_MAX,
                self.indicator_size,
                MyMessage::IndicatorSizeSlider,
            );

            iced::widget::row!(
                label,
                iced::widget::horizontal_space(SPACING_NORMAL),
                min,
                slider,
                max
            )
            .width(iced::Length::Fill)
            .align_items(iced::Alignment::Center)
            .spacing(SPACING_SMALL)
        };

        iced::widget::container(size_slider)
            .padding(SPACING_LARGE)
            .width(iced::Length::Fill)
            .into()
    }

    fn view_content(&self) -> iced::Element<'_, MyMessage> {
        let loading_indicator = iced_loading_indicator::Widget::new(
            self.indicator_size,
            Some(
                self.indicator_style
                    .unwrap_or(Default::default())
                    .to_loading_indicator_style(Some(self.current_custom_color())),
            ),
            self.dark_mode,
        )
        .tick_duration_ms(
            self.indicator_speed
                .unwrap_or(Default::default())
                .to_loading_indicator_speed(Some(self.indicator_custom_speed)),
        );

        iced::widget::container(loading_indicator)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
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
        "Detailed Loading Indicator Widget Demo".into()
    }

    fn theme(&self) -> Self::Theme {
        if self.dark_mode {
            iced::theme::Theme::Dark
        } else {
            iced::theme::Theme::Light
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        iced::widget::container(iced::widget::row!(
            self.view_sidebar(),
            iced::widget::vertical_rule(0),
            iced::widget::column!(
                self.view_header_bar(),
                iced::widget::horizontal_rule(0),
                self.view_content()
            )
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
        ))
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MyMessage::IndicatorColorInput(value, color) => match color {
                Color::R => self.indicator_custom_color_r = value,
                Color::G => self.indicator_custom_color_g = value,
                Color::B => self.indicator_custom_color_b = value,
            },
            MyMessage::IndicatorSpeedInput(value) => {
                self.indicator_custom_speed = value;
            }
            MyMessage::ResetButton => {
                self.reset();
            }
            MyMessage::IndicatorSizeSlider(value) => {
                self.indicator_size = value;
            }
            MyMessage::ThemeToggler(value) => {
                self.dark_mode = value;
            }
            MyMessage::IndicatorStylePicker(value) => {
                self.indicator_style = Some(value);
                if let IndicatorStyle::CustomColor = value {
                    return iced::widget::text_input::focus(iced::widget::text_input::Id::new(
                        INPUT_ID_CUSTOM_COLOR_R,
                    ));
                }
            }
            MyMessage::IndicatorSpeedPicker(value) => {
                self.indicator_speed = Some(value);
                return iced::widget::text_input::focus(iced::widget::text_input::Id::new(
                    INPUT_ID_CUSTOM_SPEED,
                ));
            }
        }
        iced::Command::none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum IndicatorStyle {
    #[default]
    TextColor,
    PrimaryColor,
    CustomColor,
}

impl IndicatorStyle {
    const ALL: [Self; 3] = [Self::TextColor, Self::PrimaryColor, Self::CustomColor];

    fn to_loading_indicator_style(
        &self,
        custom_color: Option<iced::Color>,
    ) -> iced_loading_indicator::Style {
        match self {
            Self::TextColor => iced_loading_indicator::Style::TextColor,
            Self::PrimaryColor => iced_loading_indicator::Style::PrimaryColor,
            Self::CustomColor => iced_loading_indicator::Style::CustomColor(
                custom_color.unwrap_or(iced::Color::from_rgb8(0xaa, 0x11, 0xff)),
            ),
        }
    }
}

impl std::fmt::Display for IndicatorStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TextColor => "Text Color",
                Self::PrimaryColor => "Primary Color",
                Self::CustomColor => "Custom Color",
            }
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum IndicatorSpeed {
    Fast,
    #[default]
    Normal,
    Slow,
    Custom,
}

impl IndicatorSpeed {
    const ALL: [Self; 4] = [Self::Fast, Self::Normal, Self::Slow, Self::Custom];

    const FAST: u64 = 50;
    const NORMAL: u64 = 100;
    const SLOW: u64 = 200;
    const CUSTOM_DEFAULT: u64 = 500;

    fn to_loading_indicator_speed(&self, custom_speed: Option<u64>) -> u64 {
        match self {
            Self::Fast => Self::FAST,
            Self::Normal => Self::NORMAL,
            Self::Slow => Self::SLOW,
            Self::Custom => custom_speed.unwrap_or(Self::CUSTOM_DEFAULT),
        }
    }
}

impl std::fmt::Display for IndicatorSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fast => format!("Fast ({} ms)", Self::FAST),
                Self::Normal => format!("Normal ({} ms)", Self::NORMAL),
                Self::Slow => format!("Slow ({} ms)", Self::SLOW),
                Self::Custom => format!("Custom"),
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    R,
    G,
    B,
}

mod numeric_input {
    // [component example](https://github.com/iced-rs/iced/blob/master/examples/component/src/main.rs)

    pub trait Unsigned: ToString + std::str::FromStr {}
    impl Unsigned for u8 {}
    impl Unsigned for u16 {}
    impl Unsigned for u32 {}
    impl Unsigned for u64 {}
    impl Unsigned for u128 {}

    pub struct NumericInput<M, T>
    where
        T: Unsigned,
    {
        placeholder: Option<String>,
        value: Option<T>,
        on_change: Box<dyn Fn(Option<T>) -> M>,
        font: Option<iced::Font>,
        size: Option<iced::Pixels>,
        id: Option<String>,
    }

    impl<M, T> NumericInput<M, T>
    where
        T: Unsigned,
    {
        pub fn new(value: Option<T>, on_change: impl Fn(Option<T>) -> M + 'static) -> Self {
            Self {
                placeholder: None,
                value,
                on_change: Box::new(on_change),
                font: None,
                size: None,
                id: None,
            }
        }

        pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
            Self {
                placeholder: Some(placeholder.into()),
                ..self
            }
        }

        pub fn font(self, font: iced::Font) -> Self {
            Self {
                font: Some(font),
                ..self
            }
        }

        pub fn size(self, size: impl Into<iced::Pixels>) -> Self {
            Self {
                size: Some(size.into()),
                ..self
            }
        }

        pub fn id(self, id: impl Into<String>) -> Self {
            Self {
                id: Some(id.into()),
                ..self
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Event {
        InputChanged(String),
    }

    impl<M, T> iced::widget::Component<M, iced::Renderer> for NumericInput<M, T>
    where
        T: Unsigned,
    {
        type State = ();
        type Event = Event;

        fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<M> {
            match event {
                Event::InputChanged(s) => {
                    if s.is_empty() {
                        Some((self.on_change)(None))
                    } else {
                        s.parse().ok().map(Some).map(self.on_change.as_ref())
                    }
                }
            }
        }

        fn view(
            &self,
            _state: &Self::State,
        ) -> iced::advanced::graphics::core::Element<'_, Self::Event, iced::Renderer> {
            let input = iced::widget::text_input(
                self.placeholder.as_ref().unwrap_or(&String::from("")),
                self.value
                    .as_ref()
                    .map(T::to_string)
                    .as_deref()
                    .unwrap_or(""),
            )
            .on_input(Event::InputChanged)
            .width(iced::Length::Fixed(50.0))
            .padding([3.0, 4.0])
            .font(self.font.unwrap_or(Default::default()))
            .size(self.size.unwrap_or(16.into()));

            if let Some(id) = self.id.as_ref() {
                input.id(iced::widget::text_input::Id::new(id.clone()))
            } else {
                input
            }
            .into()
        }
    }

    impl<'a, M, T> std::convert::From<NumericInput<M, T>> for iced::Element<'a, M, iced::Renderer>
    where
        M: 'a,
        T: Unsigned + 'a,
    {
        fn from(value: NumericInput<M, T>) -> Self {
            iced::widget::component(value)
        }
    }
}
