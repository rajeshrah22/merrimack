use iced::widget::{center, text};
use iced::window;
use iced::{Element, Subscription, Task, time};
use std::time::Duration;

fn main() -> iced::Result {
    iced::daemon(Merrimack::new, Merrimack::update, Merrimack::view)
        .title(Merrimack::title)
        .subscription(Merrimack::subscription)
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

const INTERVAL_SECONDS: u64 = 5;
const BREAK_SECONDS: u64 = 3;

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
        String::from("Merrimack - Take a Break")
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

                            let (id, open) = window::open(window::Settings::default());
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

        center(
            text(format!("Take a break!\n\n{} seconds remaining", remaining))
                .size(40)
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        let tick = time::every(Duration::from_secs(1)).map(|_| Message::Tick);
        let close_events = window::close_events().map(Message::WindowClosed);

        Subscription::batch([tick, close_events])
    }
}
