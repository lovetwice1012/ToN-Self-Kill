use enigo::{Enigo, Keyboard, Key};
use std::thread;
use std::time::Duration;

fn main() {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Other(187));
    enigo.key_down(Key::Other(222));
    thread::sleep(Duration::from_secs(5));
    enigo.key_up(Key::Other(187));
    enigo.key_up(Key::Other(222));
}