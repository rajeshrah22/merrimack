use std::{
    thread,
    time,
    sync::{Arc, Mutex},
    io::Read,
    os::unix::net::UnixListener,
};
use notify_rust::Notification;
use merrimack_common::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Merrimack!");
    println!("-- A gentle reminder for you to look away from the screen and towards the merrimack river");

    let socket_path = "/tmp/merrimack.sock";
    let _ = std::fs::remove_file("/tmp/merrimack.sock");
    let listener = UnixListener::bind(socket_path)?;

    let config = Config {
        interval_minutes: 10,
        duration_seconds: 30
    };

    let shared_config = Arc::new(Mutex::new(config));

    let config_clone1 = Arc::clone(&shared_config);
    thread::spawn(move || {
        loop {
            let config = config_clone1.lock().unwrap();
            let minutes = time::Duration::from_secs(config.interval_minutes);
            let seconds = time::Duration::from_secs(config.duration_seconds);

            println!("Sleeping for {} mintes", config.interval_minutes);
            thread::sleep(minutes);

            Notification::new()
            .summary("Break Time!")
            .body("Look away from the screen and into nature or towards Christopher Coco.")
            .show()
            .unwrap();

            thread::sleep(seconds);
            println!("breaking for {} seconds", config.duration_seconds);

            Notification::new()
            .summary("Break Time done")
            .body("Back to work.")
            .show()
            .unwrap();
        }
    });

    /*
     * recv loop
     * clean up unwraps after
     * if something goes wrong, try proper error handling next.
    */
    let config_clone2 = Arc::clone(&shared_config);
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut buffer = String::new();
        let _ = stream.read_to_string(&mut buffer).unwrap();
        if let  Ok(new_config) = serde_json::from_str::<Config>(&buffer) {
            println!("Recieved new config: {:?}", new_config);
            let mut config = config_clone2.lock().unwrap();
            *config = new_config; // does this not mutate the data?
            println!("Updated config: {:?}", config);
        } else {
            eprintln!("Invalid config JSON recieved!");
        }
    });

    loop {
        thread::park();
    }
}
