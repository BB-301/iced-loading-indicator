//! This is a Rust library that exposes a simple Iced loading
//! indicator widget that works with Iced's default renderer and
//! built-in theme, namely, [`iced_widget::core::Renderer`] and [`iced_style::Theme`],
//! respectively.
//!
//! Internally, the widget uses a canvas to draw the indicator and takes charge of the animation
//! itself by listening to [`iced_widget::core::window::Event::RedrawRequested`] events and
//! requesting redraws.
//!
//! ## Example
//! ```
#![doc = include_str!("../examples/widget.rs")]
//! ```

/// The loading indicator's coloring style.
#[derive(Default, Clone, Copy, Debug)]
pub enum Style {
    /// Based on [`iced_style::theme::Palette`]'s `text` property.
    #[default]
    TextColor,
    /// Based on [`iced_style::theme::Palette`]'s `primary` property.
    PrimaryColor,
    /// Used for a custom, user provided [`iced_widget::core::Color`].
    CustomColor(iced_widget::core::Color),
}

/// A simple tuple structure that is used to keep track
/// of the loading indicator's active (i.e. completely opact) circle.
#[derive(Clone, Copy, Debug)]
pub struct Index(usize);

impl Default for Index {
    fn default() -> Self {
        Self(0)
    }
}

impl Index {
    /// A factory method that returns an [`Index`] with its
    /// internal state set to zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// A method that mutates the internal index by increasing
    /// it by one or by resetting it back to zero if "[`NUMBER_OF_CIRCLES`]
    /// minus one" gets reached.
    pub fn tick(&mut self) {
        if self.0 == (NUMBER_OF_CIRCLES - 1) {
            self.0 = 0;
        } else {
            self.0 += 1;
        }
    }
}

/// The number of circles from which the loading indicator
/// is made up. The current value is `12` and cannot be modified.
// PRIVATE NOTES
// - Could have been: 8, (10, 12), 15, 18, 20, 24...
// - I think that 12 points is the best overall, but 10 points would have
// been Ok as well.
pub const NUMBER_OF_CIRCLES: usize = 12;

/// A private helper function used to get the index offset
/// of a point `distance` steps prior the current `index`.
fn index_offset(index: usize, distance: usize) -> usize {
    if distance >= NUMBER_OF_CIRCLES || index >= NUMBER_OF_CIRCLES {
        panic!("invalid usage");
    }
    if index >= distance {
        index - distance
    } else {
        (NUMBER_OF_CIRCLES + index) - distance
    }
}

/// The loading indicator structure, which implements the [`iced_widget::canvas::Program`]
/// trait, and which is used to draw the loading indicator for a given active index.
///
/// **NOTE** This type can be used directly inside the `Iced` application, in which case
/// the application will be responsible for implementing the animation itself
/// (see `examples/in_app.rs`), but in most cases the
/// user will prefer to rely on the [`Widget`] wrapper type, which latter will itself take care of the
/// animation (see `examples/widget.rs`).
pub struct LoadingIndicator {
    /// The loading indicator's size (in pixels).
    size: f32,
    /// The loading indicator's active index.
    index: Index,
    /// The loading indicator's style.
    style: Style,
    /// Whether to use a smaller alpha channel (`0.025` vs `0.1`) for the
    /// inactive index background color.
    lighter_inactive: bool,
}

impl LoadingIndicator {
    /// A factory method that can be used to instantiate the loading
    /// indicator with the specified `size` and `index`, with the
    /// default coloring [`Style`] and `lighter_inactive` set to `false`.
    pub fn new(size: f32, index: Index) -> Self {
        Self {
            size,
            index,
            style: Default::default(),
            lighter_inactive: false,
        }
    }

    /// A factory method that can be used to instantiate the loading
    /// indicator with the specified `size`, starting at index 0, with
    /// `lighter_inactive` set to `false`, and using the default coloring [`Style`].
    pub fn with_size(size: f32) -> Self {
        Self::new(size, Index(0))
    }

    /// A setter method that can be used to specify the coloring [`Style`].
    pub fn style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    /// A setter method that can be used to specify whether a lighter
    /// alpha channel value should be used for the background color of
    /// a circle with an inactive index. For `false`, the value `0.1` is
    /// used; for `true`, the value `0.025` is used. The latter is useful on darker
    /// backgrounds (i.e. in a darker themed app), while the former will come out
    /// nice on a lighter background (i.e. in a lighter themed app).
    pub fn lighter_inactive(self, value: bool) -> Self {
        Self {
            lighter_inactive: value,
            ..self
        }
    }
}

type Renderer = iced_widget::renderer::Renderer<iced_style::Theme>;

impl<M> iced_widget::canvas::Program<M, Renderer> for LoadingIndicator {
    type State = ();

    // See [clock example](https://github.com/iced-rs/iced/blob/master/examples/clock/src/main.rs)
    // for a nice example on how to use the Canvas with the Program trait.

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &iced_style::Theme,
        _bounds: iced_widget::core::Rectangle,
        _cursor: iced_widget::core::mouse::Cursor,
    ) -> Vec<<Renderer as iced_widget::canvas::Renderer>::Geometry> {
        let size = iced_widget::core::Size::new(self.size, self.size);

        let mut frame = iced_widget::canvas::Frame::new(renderer, size.clone());

        let center = frame.center();
        let radius = frame.width().min(frame.height()) / 2.0;

        let point_size: f32 = radius * 0.15;

        let point = iced_widget::canvas::Path::circle(
            iced_widget::core::Point::new(0.0, radius - point_size * 1.1), // NOTE: 1.1 because I noticed clipping in practice. This needs more attention...
            point_size,
        );

        frame.translate(iced_widget::core::Vector::new(center.x, center.y));

        let color = match self.style {
            Style::TextColor => theme.palette().text,
            Style::PrimaryColor => theme.palette().primary,
            // Style::PrimaryColor => theme.extended_palette().primary.strong.color, // This one would match the default Button color
            Style::CustomColor(color) => color,
        };

        let index = self.index.0;
        for i in 0..NUMBER_OF_CIRCLES {
            let color = if i == index {
                color
            } else if index_offset(index, 1) == i {
                iced_widget::core::Color { a: 0.8, ..color }
            } else if index_offset(index, 2) == i {
                iced_widget::core::Color { a: 0.6, ..color }
            } else if index_offset(index, 3) == i {
                iced_widget::core::Color { a: 0.4, ..color }
            } else if index_offset(index, 4) == i {
                iced_widget::core::Color { a: 0.20, ..color }
            } else {
                let a = if self.lighter_inactive { 0.025 } else { 0.1 };
                iced_widget::core::Color { a, ..color }
            };

            let angle_in_degrees = 360.0 / (NUMBER_OF_CIRCLES as f32);
            let angle_in_radians = angle_in_degrees * (i as f32) / 180.0 * std::f32::consts::PI;

            frame.with_save(|f| {
                f.rotate(angle_in_radians);
                f.fill(&point, color);
            });
        }

        vec![frame.into_geometry()]
    }
}

impl<'a, M> std::convert::From<LoadingIndicator> for iced_widget::core::Element<'a, M, Renderer>
where
    M: 'a + Clone,
{
    fn from(value: LoadingIndicator) -> Self {
        let s = value.size;
        Self::new(
            iced_widget::canvas(value)
                .width(iced_widget::core::Length::Fixed(s))
                .height(iced_widget::core::Length::Fixed(s)),
        )
    }
}

/// A structure used to keep track of the widget's internal state.
struct State {
    /// The loading indicator's active index.
    index: Index,
    /// The moment at which the last tick occurred.
    last_tick: std::time::Instant,
}

impl Default for State {
    fn default() -> Self {
        Self {
            index: Default::default(),
            last_tick: std::time::Instant::now(),
        }
    }
}

/// The loading indicator widget which implements
/// the [`iced_widget::core::Widget`] trait and which acts as a
/// convenient wrapper around the [`LoadingIndicator`] type, taking
/// care of the animation for maximal convenience (i.e. instead of requiring
/// the user to manually implement the animation himself inside the application).
pub struct Widget<'a, M> {
    /// The loading indicator's size in pixels.
    size: f32,
    /// The "tick interval" (i.e. animation speed), in milliseconds, used
    /// by the widget. If not provided, this value will default to [`Self::DEFAULT_TICK_DURATION_MS`].
    tick_duration_ms: u64,
    /// A vector of [`iced_widget::canvas::Canvas`] items (containing [`LoadingIndicator`]s)
    /// converted into [`iced_widget::core::Element`]s.
    content: Vec<iced_widget::core::Element<'a, M, Renderer>>,
}

impl<'a, M> Widget<'a, M> {
    /// The default "tick interval" (i.e. animation speed), in milliseconds, used
    /// by the widget if none gets specified.
    pub const DEFAULT_TICK_DURATION_MS: u64 = 80;
}

impl<'a, M> Widget<'a, M>
where
    M: 'a + Clone,
{
    /// The factory method that must be used to instantiate the widget.
    ///
    /// **Parameters**:
    /// * `size`: The indicator's size in pixels.
    /// * `style`: An optional value containing the [`LoadingIndicator`]'s style to be used.
    /// * `lighter_inactive`: A boolean value indicating whether the [`LoadingIndicator`] should
    /// use a `0.1` alpha channel (`false`) or a `0.025` alpha channel (`true`) for the background
    /// in inactive circle indexes. The latter comes out better in darker themed apps, while the
    /// former is better for lighter themed apps.
    pub fn new(size: f32, style: Option<Style>, lighter_inactive: bool) -> Self {
        let content = (0usize..NUMBER_OF_CIRCLES)
            .into_iter()
            .map(|index| {
                LoadingIndicator::new(size, Index(index))
                    .style(style.unwrap_or(Default::default()))
                    .lighter_inactive(lighter_inactive)
                    .into()
            })
            .collect();
        Self {
            size,
            content,
            tick_duration_ms: Self::DEFAULT_TICK_DURATION_MS,
        }
    }

    /// A setter method that can be used to specify the animation speed (in milliseconds).
    pub fn tick_duration_ms(self, value: u64) -> Self {
        Self {
            tick_duration_ms: value,
            ..self
        }
    }
}

impl<'a, M> iced_widget::core::Widget<M, Renderer> for Widget<'a, M>
where
    M: 'a + Clone,
{
    fn width(&self) -> iced_widget::core::Length {
        iced_widget::core::Length::Fixed(self.size)
    }

    fn height(&self) -> iced_widget::core::Length {
        iced_widget::core::Length::Fixed(self.size)
    }

    fn tag(&self) -> iced_widget::core::widget::tree::Tag {
        iced_widget::core::widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> iced_widget::core::widget::tree::State {
        iced_widget::core::widget::tree::State::new(State::default())
    }

    fn children(&self) -> Vec<iced_widget::core::widget::Tree> {
        self.content
            .iter()
            .map(iced_widget::core::widget::tree::Tree::new)
            .collect()
    }

    fn draw(
        &self,
        tree: &iced_widget::core::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_widget::core::Renderer>::Theme,
        style: &iced_widget::core::renderer::Style,
        layout: iced_widget::core::Layout<'_>,
        cursor: iced_widget::core::mouse::Cursor,
        viewport: &iced_widget::core::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        self.content[state.index.0].as_widget().draw(
            &tree.children[state.index.0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &iced_widget::core::layout::Limits,
    ) -> iced_widget::core::layout::Node {
        let limits = limits.width(self.size).height(self.size);
        let size = limits.resolve(iced_widget::core::Size::ZERO);
        iced_widget::core::layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        tree: &mut iced_widget::core::widget::Tree,
        event: iced_widget::core::Event,
        _layout: iced_widget::core::Layout<'_>,
        _cursor: iced_widget::core::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_widget::core::Clipboard,
        shell: &mut iced_widget::core::Shell<'_, M>,
        _viewport: &iced_widget::core::Rectangle,
    ) -> iced_widget::core::event::Status {
        let state = tree.state.downcast_mut::<State>();

        if let iced_widget::core::Event::Window(
            iced_widget::core::window::Event::RedrawRequested(now),
        ) = event
        {
            let delay = std::time::Duration::from_millis(self.tick_duration_ms);
            let elapsed = now.duration_since(state.last_tick);

            if elapsed > delay {
                state.index.tick();
                state.last_tick = now;
                shell.request_redraw(iced_widget::core::window::RedrawRequest::At(now + delay));
            } else {
                let remaining = delay - elapsed;
                shell.request_redraw(iced_widget::core::window::RedrawRequest::At(
                    now + remaining,
                ))
            }
        }

        iced_widget::core::event::Status::Ignored
    }
}

impl<'a, M> std::convert::From<Widget<'a, M>> for iced_widget::core::Element<'a, M, Renderer>
where
    M: 'a + Clone,
{
    fn from(value: Widget<'a, M>) -> Self {
        Self::new(value)
    }
}
