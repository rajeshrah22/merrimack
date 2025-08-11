use std::fs;
use std::io::Read;
use std::os::fd::AsFd;
use std::os::unix::net::UnixListener;

use nix::{
    poll::{PollFd, PollFlags, PollTimeout, poll},
    sys::{
        time::{TimeSpec, TimeValLike},
        timer::Expiration,
        timer::TimerSetTimeFlags,
        timerfd::{ClockId, TimerFd, TimerFlags},
    },
};

use merrimack_common::Config;

fn main() {
    let mut current_config = merrimack_common::Config::default();
    current_config.interval_minutes = 1;
    current_config.duration_seconds = 5;

    println!("Welcome to Merrimack!");
    println!(
        "-- A gentle reminder for you to look away from the screen and towards the merrimack river"
    );

    let clockid = ClockId::CLOCK_MONOTONIC;
    const SOCK_PATH: &str = "/tmp/merrimack-config";
    if let Err(e) = fs::remove_file(SOCK_PATH) {
        eprintln!("Failed to delete socket file: {}", e);
    }
    let config_listener = UnixListener::bind(SOCK_PATH).unwrap();
    let config_fd = config_listener.as_fd();

    let break_timer = TimerFd::new(clockid, TimerFlags::empty()).unwrap();
    let break_timer_fd = break_timer.as_fd();

    let interval_timer = TimerFd::new(clockid, TimerFlags::empty()).unwrap();
    let interval_timer_fd = interval_timer.as_fd();
    interval_timer
        .set(
            Expiration::Interval(TimeSpec::seconds(
                current_config.interval_minutes as i64 * 60,
            )),
            TimerSetTimeFlags::empty(),
        )
        .unwrap();

    let mut pollfds = [
        PollFd::new(break_timer_fd, PollFlags::POLLIN),
        PollFd::new(interval_timer_fd, PollFlags::POLLIN),
        PollFd::new(config_fd, PollFlags::POLLIN),
    ];

    loop {
        let nready = poll(&mut pollfds, PollTimeout::NONE).unwrap();
        assert!(nready >= 1);

        // break timer event
        if pollfds[0]
            .revents()
            .unwrap_or(PollFlags::empty())
            .contains(PollFlags::POLLIN)
        {
            break_timer.wait().unwrap();
            break_timer.unset().unwrap();
            println!("break done");
        }

        // interval timer event
        if pollfds[1]
            .revents()
            .unwrap_or(PollFlags::empty())
            .contains(PollFlags::POLLIN)
        {
            interval_timer.wait().unwrap();
            break_timer
                .set(
                    Expiration::Interval(TimeSpec::seconds(current_config.duration_seconds as i64)),
                    TimerSetTimeFlags::empty(),
                )
                .unwrap();
            println!("break start");
        }

        if pollfds[2]
            .revents()
            .unwrap_or(PollFlags::empty())
            .contains(PollFlags::POLLIN)
        {
            match config_listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = String::new();

                    stream.read_to_string(&mut buf).unwrap();

                    let new_config: Config = serde_json::from_str(buf.as_str()).unwrap_or_default();

                    if new_config.interval_minutes > 0 && new_config.duration_seconds > 0 {
                        current_config.interval_minutes = new_config.interval_minutes;
                        current_config.duration_seconds = new_config.duration_seconds;

                        println!(
                            "New config: {} minute interval, {} second duration.",
                            current_config.interval_minutes, current_config.duration_seconds
                        );

                        // set new timer
                        interval_timer
                            .set(
                                Expiration::Interval(TimeSpec::seconds(
                                    current_config.interval_minutes as i64 * 60,
                                )),
                                TimerSetTimeFlags::empty(),
                            )
                            .unwrap();

                        continue;
                    } else if new_config.duration_seconds == 0 {
                        eprintln!("Duration seconds is 0, not allowed.")
                    } else if new_config.interval_minutes == 0 {
                        eprintln!("Interval minuts is 0, not allowed.")
                    }
                }
                Err(e) => eprintln!("Failed to accept config: {}", e),
            }
        }
    }
}
