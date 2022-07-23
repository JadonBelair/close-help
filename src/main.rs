use enigo::{Enigo, KeyboardControllable, Key};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

fn main() {

    let mut enigo = Enigo::new();
    let mut rand = thread_rng();

    let range = 30..=600;

    let mut timer = Instant::now();
    let mut delay = Duration::from_secs(rand.gen_range(range.clone()));

    loop {
        if timer.elapsed() >= delay {
            enigo.key_down(Key::Alt);
            enigo.key_down(Key::F4);

            enigo.key_up(Key::Alt);
            enigo.key_up(Key::F4);
            
            timer = Instant::now();
            delay = Duration::from_secs(rand.gen_range(range.clone()));
        }
    }

}
