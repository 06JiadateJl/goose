// Variant records: see bench/goose/records_var.goose. A Rust `enum` is the
// single obvious way to write this, and it is a good one -- the discriminant is
// a byte and the compiler finds a niche for it inside the `String` payload -- but
// it is still a fixed enum: every element of the `Vec` is as large as the
// largest variant, and the `Say` text is a separate heap allocation on top. That
// is the shape records_fixed.goose exists to be the control for.
mod bench;
use bench::*;

const N: i64 = 500000;                     // BENCH_N
const PASSES: i64 = 4;

#[derive(Clone, Copy)]
enum Event {
    Tick,
    Move { id: i32, dx: i16, dy: i16 },
    Say { id: i32, text: [u8; 16], len: usize },
    Quit { id: i32, code: u8 },
}

// Генерация слова в фиксированный буфер вместо создания String
fn word(seed: u64) -> ([u8; 16], usize) {
    let mut w = [0u8; 16];
    let mut r = xs_next(seed);
    let n = 4 + xs_mod(r, 12);
    for i in 0..n {
        r = xs_next(r);
        w[i as usize] = (b'a' + xs_mod(r, 26) as u8);
    }
    (w, n as usize)
}

fn main() {
    // Выделяем память заранее, избегая реаллокаций
    let mut log: Vec<Event> = Vec::with_capacity(N as usize);
    let mut r: u64 = 12345;
    for _ in 0..N {
        r = xs_next(r);
        let k = xs_mod(r, 8);
        let id = xs_mod(r >> 3, 100000) as i32;
        if k < 4 {
            log.push(Event::Move {
                id,
                dx: (xs_mod(r >> 20, 201) - 100) as i16,
                dy: (xs_mod(r >> 40, 201) - 100) as i16,
            });
        } else if k < 6 {
            let (text, len) = word(r);
            log.push(Event::Say { id, text, len });
        } else if k < 7 {
            log.push(Event::Quit {
                id,
                code: xs_mod(r >> 20, 256) as u8,
            });
        } else {
            log.push(Event::Tick);
        }
    }
    let mut total: i64 = 0;
    for _ in 0..PASSES {
        for e in &log {
            total += match *e {
                Event::Tick => 1,
                Event::Move { id, dx, dy } => id as i64 + dx as i64 + dy as i64,
                Event::Say { id, text, len } => id as i64 + len as i64 + text[0] as i64,
                Event::Quit { id, code } => id as i64 + code as i64,
            };
        }
    }
    emit(log.len() as i64);
    emit(total);
}
