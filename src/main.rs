use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::Duration;

fn main() {
    enigo::key(Key::Other(187), Press).unwrap();
    enigo::key(Key::Other(222), Press).unwrap();
    thread::sleep(Duration::from_secs(5));
    enigo::key(Key::Other(187), Release).unwrap();
    enigo::key(Key::Other(222), Release).unwrap();
}
