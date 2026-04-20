use iced::widget::{center, column, container, text};
use iced::window;
use iced::{Color, Element, Fill, Size, Subscription, Task, time, theme};
use std::time::Duration;

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
    iced::daemon(Merrimack::new, Merrimack::update, Merrimack::view)
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
}

const INTERVAL_SECONDS: u64 = 10;
const BREAK_SECONDS: u64 = 5;

impl Merrimack {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: State::Waiting,
                overlay_window: None,
                seconds_elapsed: 0,
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
                        if self.seconds_elapsed >= INTERVAL_SECONDS {
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
                        if self.seconds_elapsed >= BREAK_SECONDS {
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
                println!("Window closed: {:?}", id);
                if self.overlay_window == Some(id) {
                    self.overlay_window = None;
                    self.state = State::Waiting;
                    self.seconds_elapsed = 0;
                }
                Task::none()
            }
        }
    }

    fn view(&self, _window: window::Id) -> Element<'_, Message> {
        let remaining = BREAK_SECONDS.saturating_sub(self.seconds_elapsed);

        let message = text("Look away from the screen.")
            .size(48)
            .color(TEXT_COLOR);

        let subtitle = text("Rest your eyes. Stretch a little.")
            .size(24)
            .color(Color { a: 0.6, ..TEXT_COLOR });

        let countdown = text(format!("{remaining}"))
            .size(72)
            .color(TEXT_COLOR);

        let content = column![message, subtitle, countdown]
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

        Subscription::batch([tick, close_events])
    }
}
