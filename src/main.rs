use clap::Parser;
use iced::keyboard;
use iced::widget::{center, column, container, text};
use iced::window;
use iced::{Color, Element, Fill, Size, Subscription, Task, time, theme};
use std::time::Duration;

#[derive(Parser)]
#[command(about = "A gentle reminder to look away from the screen")]
struct Args {
    /// Interval between breaks in minutes
    #[arg(long, default_value_t = 20)]
    interval: u64,
    /// Break duration in seconds
    #[arg(long, default_value_t = 30)]
    duration: u64,
}

const OVERLAY_BG: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.85,
};

const TEXT_COLOR: Color = Color {
    r: 0.9,
    g: 0.9,
    b: 0.9,
    a: 1.0,
};

fn main() -> iced::Result {
    let args = Args::parse();
    let interval_secs = args.interval * 60;
    let break_secs = args.duration;

    iced::daemon(
        move || Merrimack::new(interval_secs, break_secs),
        Merrimack::update,
        Merrimack::view,
    )
    .title(Merrimack::title)
    .subscription(Merrimack::subscription)
    .style(|_state, _theme| theme::Style {
        background_color: Color::TRANSPARENT,
        text_color: TEXT_COLOR,
    })
    .run()
}

struct Merrimack {
    state: State,
    overlay_window: Option<window::Id>,
    seconds_elapsed: u64,
    interval_secs: u64,
    break_secs: u64,
}

enum State {
    Waiting,
    OnBreak,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    Dismiss,
}

impl Merrimack {
    fn new(interval_secs: u64, break_secs: u64) -> (Self, Task<Message>) {
        println!(
            "Merrimack: {} min interval, {} sec break",
            interval_secs / 60,
            break_secs
        );
        (
            Self {
                state: State::Waiting,
                overlay_window: None,
                seconds_elapsed: 0,
                interval_secs,
                break_secs,
            },
            Task::none(),
        )
    }

    fn title(&self, _window: window::Id) -> String {
        String::from("Merrimack")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.seconds_elapsed += 1;

                match self.state {
                    State::Waiting => {
                        if self.seconds_elapsed >= self.interval_secs {
                            self.state = State::OnBreak;
                            self.seconds_elapsed = 0;

                            let (id, open) = window::open(window::Settings {
                                size: Size::new(1920.0, 1080.0),
                                decorations: false,
                                transparent: true,
                                level: window::Level::AlwaysOnTop,
                                resizable: false,
                                ..window::Settings::default()
                            });
                            self.overlay_window = Some(id);
                            return open.map(Message::WindowOpened);
                        }
                    }
                    State::OnBreak => {
                        if self.seconds_elapsed >= self.break_secs {
                            self.state = State::Waiting;
                            self.seconds_elapsed = 0;

                            if let Some(id) = self.overlay_window.take() {
                                return window::close(id);
                            }
                        }
                    }
                }

                Task::none()
            }
            Message::WindowOpened(id) => {
                println!("Break window opened: {:?}", id);
                Task::none()
            }
            Message::WindowClosed(id) => {
                if self.overlay_window == Some(id) {
                    self.overlay_window = None;
                    self.state = State::Waiting;
                    self.seconds_elapsed = 0;
                }
                Task::none()
            }
            Message::Dismiss => {
                if let Some(id) = self.overlay_window.take() {
                    self.state = State::Waiting;
                    self.seconds_elapsed = 0;
                    return window::close(id);
                }
                Task::none()
            }
        }
    }

    fn view(&self, _window: window::Id) -> Element<'_, Message> {
        let remaining = self.break_secs.saturating_sub(self.seconds_elapsed);

        let message = text("Look away from the screen.")
            .size(48)
            .color(TEXT_COLOR);

        let subtitle = text("Rest your eyes. Stretch a little.")
            .size(24)
            .color(Color { a: 0.6, ..TEXT_COLOR });

        let countdown = text(format!("{remaining}"))
            .size(72)
            .color(TEXT_COLOR);

        let hint = text("Press Escape or Space to dismiss")
            .size(16)
            .color(Color { a: 0.4, ..TEXT_COLOR });

        let content = column![message, subtitle, countdown, hint]
            .spacing(20)
            .align_x(iced::Center);

        container(center(content))
            .width(Fill)
            .height(Fill)
            .style(|_theme| container::Style {
                background: Some(OVERLAY_BG.into()),
                ..container::Style::default()
            })
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = time::every(Duration::from_secs(1)).map(|_| Message::Tick);
        let close_events = window::close_events().map(Message::WindowClosed);

        let keys = keyboard::listen().filter_map(|event| {
            use keyboard::key;
            let keyboard::Event::KeyPressed { modified_key, .. } = event else {
                return None;
            };
            match modified_key.as_ref() {
                keyboard::Key::Named(key::Named::Escape | key::Named::Space) => {
                    Some(Message::Dismiss)
                }
                _ => None,
            }
        });

        Subscription::batch([tick, close_events, keys])
    }
}
