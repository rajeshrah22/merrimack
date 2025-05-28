use nix::{
    poll::{poll, PollFd, PollFlags},
    sys::signal,
    sys::signalfd::SignalFd,
    sys::timerfd::{ClockId, Expiration, TimerFd, TimerFlags, TimerSetTimeFlags},
};
use std::{
    fs,
    io::Read,
    os::unix::io::{AsRawFd, RawFd},
    os::unix::net::UnixListener,
    time::Duration,
};
use notify_rust::Notification;
use merrimack_common::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Merrimack!");
    println!("-- A gentle reminder for you to look away from the screen and towards the merrimack river");

    Ok(())
}
