use std::io::Write;


#[derive(Debug, Copy, Clone)]
struct PixelState {
    hvis: bool,
    hsync: bool,
    vvis: bool,
    vsync: bool,
    frame_vis: bool,
    end: bool
}

impl Default for PixelState {
    fn default() -> Self {
        PixelState { hvis: false, hsync: true, vvis: false, vsync: true, frame_vis: false, end: false }
    }
}

impl From<PixelState> for u8 {
    fn from(p: PixelState) -> u8 {
        p.hvis as u8 | (p.hsync as u8) << 1 | (p.vvis as u8) << 2 | (p.vsync as u8) << 3 | (p.frame_vis as u8) << 4 | (p.end as u8) << 5
    }
}

const WIDTH: usize = 800 / 16;
const HEIGHT: usize = 525;


fn main() {
    let mut timing = [PixelState::default(); WIDTH * HEIGHT + 1];

    for line in 0..HEIGHT {
        for pixel in 0..WIDTH {
            let i = line * WIDTH + pixel;
            let mut state = PixelState::default();
            let mut repchar = '_';
            if pixel < 40 {
                state.hvis = true;
            }
            if pixel >= 41 && pixel < 47 {
                // active low
                state.hsync = false;
            }
            
            if line >= 40 && line < 440 {
                state.vvis = true;
            }
            if line >= 490 && line < 492 {
                // active low
                state.vsync = false;
            }
            state.frame_vis = state.hvis && state.vvis;
            if state.frame_vis {
                repchar = '.';
            }
            if !state.hsync || !state.vsync {
                repchar = '^';
            }
            eprint!("{}", repchar);
            timing[i] = state;
        }
        eprintln!();
    }
    let endstate = PixelState { end: true, ..Default::default() };
    timing[WIDTH * HEIGHT] = endstate;

    eprintln!("{:?}", timing[WIDTH * HEIGHT - 1]);
    eprintln!("{:?}", timing[WIDTH * HEIGHT]);

    let mut prom_file = [0xff as u8; 32768];
    for i in 0..timing.len() {
        prom_file[i] = timing[i].into();
        if i == WIDTH * HEIGHT - 1 {
            eprintln!("i = {}: byte: {}", i, prom_file[i]);
        }
        if i == WIDTH * HEIGHT {
            eprintln!("i = {}: byte: {}", i, prom_file[i]);
        }
    }

    let _ = std::io::stdout().write_all(&prom_file);

}
