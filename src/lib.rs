use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::thread;
use std::time::Duration;

#[no_mangle]
pub extern "C" fn press_keys() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    enigo.key(Key::Other(187), Press).unwrap();  // F1
    enigo.key(Key::Other(222), Press).unwrap();  // F2
    thread::sleep(Duration::from_secs(5));
    enigo.key(Key::Other(187), Release).unwrap();
    enigo.key(Key::Other(222), Release).unwrap();
}

