use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::process::Command;
use std::process::Child;
use std::io;
use std::mem::drop;

// function that runs the shell command 'echo hello world'
fn dispatch(url: String) -> Child {
    let child = Command::new("youtube-dl")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--output")
        .arg("./downloads/%(title)s.%(ext)s")
        .arg(url)
        .spawn()
        .unwrap();
    return child;
}

fn incrementer(counter: Arc<Mutex<u32>>, url: String) {
    let mut child = dispatch(url);
    let mut reported_counter = 0;
    let mut incremented_num = false;
    while child.try_wait().unwrap().is_none() {
        let mut num = counter.lock().unwrap();
        if !incremented_num {
           *num += 1;
            incremented_num = true;
        }
        if reported_counter != *num {
            println!("{} downloads remaining", num);
            reported_counter = *num;
        }

        drop(num);
        thread::sleep(Duration::from_millis(1));
    }
    let mut num = counter.lock().unwrap();
    *num -= 1;
    println!("Completed donwload, {} remaining.", *num);
}


fn main() {
    let downloads = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mut user_input = String::new();

    let mut quit = false;
    while !quit {
        println!("Enter a Youtube URL:");
        user_input.clear();
        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                match user_input.trim() {
                    "quit" => {
                        quit = true;
                    }
                    _ => {
                        let counter = Arc::clone(&downloads);
                        let user_copy = String::clone(&user_input);
                        println!("Spawning new thread...");
                        let handle = thread::spawn(move || incrementer(counter, user_copy));
                        handles.push(handle);
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Downloaded: {}", *downloads.lock().unwrap());
}
