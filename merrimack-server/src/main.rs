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

fn main() {
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

    let timer = TimerFd::new(clockid, TimerFlags::empty()).unwrap();
    let timer_fd = timer.as_fd();
    timer
        .set(
            Expiration::Interval(TimeSpec::seconds(5)),
            TimerSetTimeFlags::empty(),
        )
        .unwrap();

    let mut pollfds = [
        PollFd::new(timer_fd, PollFlags::POLLIN),
        PollFd::new(config_fd, PollFlags::POLLIN),
    ];

    loop {
        let nready = poll(&mut pollfds, PollTimeout::NONE).unwrap();
        assert!(nready >= 1);

        if pollfds[0]
            .revents()
            .unwrap_or(PollFlags::empty())
            .contains(PollFlags::POLLIN)
        {
            timer.wait().unwrap();
            println!("timer done");
        }

        if pollfds[1]
            .revents()
            .unwrap_or(PollFlags::empty())
            .contains(PollFlags::POLLIN)
        {
            match config_listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = String::new();
                    stream.read_to_string(&mut buf).unwrap();
                    println!("Recieved config: {}", buf.trim());
                }
                Err(e) => eprintln!("Failed to accept config: {}", e),
            }
        }
    }
}
