use crate::{InterruptHandler,print,println};
use pc_keyboard::DecodedKey;

pub struct DemoHandler {
    ticks: usize
}

impl InterruptHandler for DemoHandler {
    fn new() -> Self {
        DemoHandler {ticks: 0}
    }

    fn first_tick(&mut self) {
        println!("DemoHandler started!");
        self.regular_tick();
    }

    fn regular_tick(&mut self) {
        self.ticks += 1;
        print!("{}; ", self.ticks);
    }

    fn key(&mut self, key: Option<DecodedKey>) {
        if let Some(key) = key {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
}