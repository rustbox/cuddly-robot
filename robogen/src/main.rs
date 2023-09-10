use std::io::Write;

use bitflags::bitflags;

bitflags! {
    /// This is state of the vga timing at some "wide" pixel column.
    /// The states expressed here are all in terms of "active high"
    /// to make the logic easier to understand. 
    /// For example, at pixel 0 is horizontally visible, so HVIS is set.
    /// At pixel 41 it's in the blanking area (not visible) but the
    /// horizontal sync pulse should be active, so HVIS is not set and
    /// HSYNC is set.
    struct PixelState: u8 {
        const HVIS = 1;
        const HSYNC = 1 << 1;
        const VVIS = 1 << 2;
        const VSYNC = 1 << 3;
        const FRAME_VIS = 1 << 4;
        const FRAME_NOT_VIS = 1 << 5;
        const START_FRAME_WRITE = 1 << 6;
        const END = 1 << 7;
    }
}

impl PixelState {
    /// These are the bits that are active low. So when converting to a byte
    /// we can flip the bits into an active low domain, rather than the active high
    fn active_low() -> PixelState {
        PixelState::HSYNC | PixelState::VSYNC | PixelState::END
    }
}


impl Default for PixelState {
    fn default() -> Self {
        PixelState::HVIS | PixelState::VVIS
    }
}

impl From<PixelState> for u8 {
    fn from(p: PixelState) -> u8 {
        (PixelState::active_low() ^ p).bits()
    }
}

const WIDTH: usize = 800 / 16;
const HEIGHT: usize = 525;

fn main() {
    let mut timing = [PixelState::default(); WIDTH * HEIGHT];

    for line in 0..HEIGHT {
        for pixel in 0..WIDTH {
            let i = line * WIDTH + pixel;

            let mut state = PixelState::empty();
            state.set(PixelState::HVIS, pixel < 40);
            state.set(PixelState::HSYNC, pixel >= 41 && pixel < 47);
            state.set(PixelState::VVIS, line >= 40 && line < 440);
            state.set(PixelState::VSYNC, line >= 490 && line < 492);
            state.set(PixelState::FRAME_VIS, state.contains(PixelState::HVIS) && state.contains(PixelState::VVIS));
            state.set(PixelState::FRAME_NOT_VIS, !state.contains(PixelState::FRAME_VIS));
            state.set(PixelState::START_FRAME_WRITE, i != 1488);
            state.set(PixelState::END, i == WIDTH * HEIGHT - 1);
            
            let mut repchar = '_';
            if state.contains(PixelState::FRAME_VIS) {
                repchar = '.';
            }

            if state.contains(PixelState::HSYNC) || state.contains(PixelState::VSYNC) {
                repchar = '^';
            }

            if state.contains(PixelState::END) {
                repchar = 'x';
            }

            if !state.contains(PixelState::START_FRAME_WRITE) {
                repchar = '*'
            }

            eprint!("{}", repchar);
            timing[i] = state;
        }
        eprintln!();
    }
    // timing[WIDTH * HEIGHT] = PixelState::END;

    eprintln!("end: {}", timing[WIDTH * HEIGHT - 1].bits);

    let mut prom_file = [0xff as u8; 32768];
    for i in 0..timing.len() {
        prom_file[i] = timing[i].into();
    }

    let _ = std::io::stdout().write_all(&prom_file);

}
