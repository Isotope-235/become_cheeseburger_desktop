use crate::*;
pub async fn find_fps() -> i32 {
    let mut attempts = 0;
    'fps: loop {
        attempts += 1;
        let mean = try_find_fps().await;
        match mean { // accept number if it is close to a common refresh rate
            25..=34 |
            55..=64 |
            85..=94 |
            115..=124 |
            139..=148 |
            235..=244 => break 'fps mean,
            _ => if attempts > 3 { panic!("unable to find stable fps") } else { continue; }
        }
    }
}

async fn try_find_fps() -> i32 {
    let mut frames = Vec::new();
    for _ in 0..16 { // get fps numbers from the first 16 frames
        clear_background(BG);
        next_frame().await;
        frames.push(get_fps());
    }
    let mut adjusted: Vec<_> = frames.iter().skip(8).collect(); // early fps numbers are unreliable
    adjusted.sort_unstable();
    let mean = *adjusted[7];
    mean
}