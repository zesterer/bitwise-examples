use bitwise_challenge::{Game, Input, Output, Key};

struct Snake;

const CELLS: u32 = 8;
const CELL: u32 = 32;
const SCORE_H: u32 = 64;

struct Data {
    pos: [u32; 2],
    dir: u32,
    score: u8,
    fruit_pos: [u32; 2],
    tail: [u8; 19],
    is_dead: bool,
}

fn rand(seed: u64) -> u64 {
    let x = (seed.wrapping_mul(182099923) ^ seed).wrapping_add(8301719803) ^ seed;
    x ^ seed ^ x.wrapping_div(21273)
}

const fn make_state(data: Data) -> u64 {
    let pos = (data.pos[1] * CELLS + data.pos[0]) as u64 & 0b11111111; // 0
    let dir = ((data.dir & 0b11) << 8) as u64; // 8
    let score = (data.score as u64) << 10; // 7
    let fruit_pos = (((data.fruit_pos[1] * CELLS + data.fruit_pos[0]) & 0b11111111) as u64) << 17; // 17
    // 25
    let mut tail = 0;
    let mut i = 0;
    while i < 19 {
        tail |= ((data.tail[i] & 0b11) as u64) << (25 + i * 2);
        i += 1;
    }
    // 63
    let dead = if data.is_dead { 1 } else { 0 } << 63;

    pos | dir | score | fruit_pos | tail | dead
}

fn from_state(state: u64) -> Data {
    Data {
        // 0
        pos: [
            (state & 0b11111111) as u32 % CELLS,
            (state & 0b11111111) as u32 / CELLS,
        ],
        // 8
        dir: ((state >> 8) & 0b11) as u32,
        // 10
        score: ((state >> 10) & 0b111111) as u8,
        // 17
        fruit_pos: [
            ((state >> 17) & 0b11111111) as u32 % CELLS,
            ((state >> 17) & 0b11111111) as u32 / CELLS,
        ],
        // 25
        tail: {
            let mut tail = [0; 19];
            for i in 0..19 {
                tail[i] = ((state >> (25 + i as u64 * 2)) & 0b11) as u8;
            }
            tail
        },
        is_dead: ((state >> 63) & 0b1) == 1,
    }
}

impl Game for Snake {
    const NAME: &'static str = "Snake";
    const WIDTH: usize = (CELLS * CELL) as usize;
    const HEIGHT: usize = (CELLS * CELL + SCORE_H) as usize;

    fn init() -> u64 {
        make_state(Data {
            pos: [4, 4],
            dir: 0,
            score: 0,
            fruit_pos: [5, 3],
            tail: [0; 19],
            is_dead: false,
        })
    }

    fn tick(prev: u64, input: &Input<'_, Self>, output: &mut Output<'_, Self>) -> u64 {
        let mut data = from_state(prev);

        fn move_dir(mut pos: [u32; 2], dir: u32) -> [u32; 2] {
            match dir {
                0 => pos[0] = (pos[0] + 1) % CELLS,
                1 => pos[1] = (pos[1] + CELLS - 1) % CELLS,
                2 => pos[0] = (pos[0] + CELLS - 1) % CELLS,
                3 => pos[1] = (pos[1] + 1) % CELLS,
                _ => unreachable!(),
            }
            pos
        }

        if input.tick() % 15 == 0 && !data.is_dead {
            data.pos = move_dir(data.pos, data.dir);

            if data.pos == data.fruit_pos {
                let x = rand(input.tick());
                let y = rand(x);
                data.fruit_pos = [x as u32 % CELLS, y as u32 % CELLS];
                data.score += 1;
            }

            for i in (0..18).rev() {
                data.tail[i + 1] = data.tail[i];
            }
            data.tail[0] = (data.dir as u8 + 2) % 4;
        }


        let new_dir = if input.is_key_down(Key::Right) {
            0
        } else if input.is_key_down(Key::Left) {
            2
        } else if input.is_key_down(Key::Up) {
            1
        } else if input.is_key_down(Key::Down) {
            3
        } else {
            data.dir
        };
        if new_dir != (data.dir + 2) % 4 {
            data.dir = new_dir;
        }

        if !data.is_dead {
            // Draw snake
            let mut segment = data.pos;
            for i in 0..data.score + 1 {
                if i > 0 && segment == data.pos {
                    data.is_dead = true;
                    data.score = 0;
                }

                output.rect((segment[0] * CELL) as i32, (segment[1] * CELL + SCORE_H) as i32, CELL, CELL, [0, (i as u8) * 10, 255 - (i as u8) * 10]);
                if let Some(dir) = data.tail.get(i as usize) {
                    segment = move_dir(segment, *dir as u32);
                } else {
                    break;
                }
            }
        }

        // Draw fruit
        output.rect((data.fruit_pos[0] * CELL) as i32, (data.fruit_pos[1] * CELL + SCORE_H) as i32, CELL, CELL, [0, 255, 0]);

        // Draw score
        if data.is_dead {
            output.rect(0, 0, CELLS * CELL, SCORE_H, [0, 0, if input.tick() % 16 < 8 { 255 } else { 0 }]);
            data.score += 1;
            if data.score == 63 {
                return Self::init();
            }
        } else {
            output.rect(0, 0, CELLS * CELL, SCORE_H, [100, 100, 100]);
            output.rect(0, 0, data.score as u32 * 5, SCORE_H, [0, 255, 0]);
        }

        make_state(data)
    }
}


fn main() { Snake::run() }
