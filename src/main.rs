use std::{thread, time};
use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Merrimack!");
    println!("-- A gentle reminder for you to look away from the screen and towards the merrimack river");

    let minutes = time::Duration::from_secs(10 * 60);
    let seconds = time::Duration::from_secs(30);


    loop {
        thread::sleep(minutes);

        Notification::new()
        .summary("Break Time!")
        .body("Look away from the screen and into nature or towards Christopher Coco.")
        .show()?;

        thread::sleep(seconds);

        Notification::new()
        .summary("Break Time done")
        .body("Back to work.")
        .show()?;
    }
}
