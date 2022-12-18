use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum Signal {
    High,
    Low,
    Z,
}

impl Signal {
    pub fn invert(&self) -> Signal {
        match &self {
            Signal::High => Signal::Low,
            Signal::Low => Signal::High,

            Signal::Z => todo!("what's the inverse of high-Z? more high-Z probably? Or, do clocks run on a different Signal that can't be high-Z?"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bus {
    lines: Vec<Signal>,
}

#[derive(Debug, Clone)]
pub enum State {
    Signal(Signal),
    Bus(Bus),
}

// fugit::Instant can only do u32, u64. Do we need signed? idk
// type Instant = fugit::Instant<i64, 1, 1>;

type Instant = i64; // what, femtoseconds?
type Duration = i64; // what, femtoseconds?
type Step = i64; // what, femtoseconds?

// see https://stackoverflow.com/questions/27886474/recursive-function-type
// TODO: generator? https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html
pub struct StateFn(Box<dyn FnOnce() -> (State, Vec<(Step, StateFn)>)>);

// TODO: would something like this be better?
#[allow(dead_code)]
pub struct _State {
    state: State,
    next: Box<dyn FnOnce() -> Vec<(Step, _State)>>,
}

fn main() {
    // big idea: event-based simulator that tracks history,
    // which gets mapped to https://github.com/wavedrom/schema/blob/master/WaveJSON.md

    // things to model:
    // clock (square wave)
    // inverter (non-GAL)
    // SIPO (pretending it's not a GAL)
    // FIFO

    // An ideal square clock
    // (do we need to model rise/fall times?)
    fn make_clock(initial_state: Signal, pulse_width: Duration) -> StateFn {
        let clock = move || -> (State, Vec<(Step, StateFn)>) {
            return (
                State::Signal(initial_state),
                vec![(pulse_width, make_clock(initial_state.invert(), pulse_width))],
            );
        };
        return StateFn(Box::new(clock));
    }

    let mut total_steps = 6;
    let mut simulate = move || -> bool {
        total_steps -= 1;
        total_steps > 0
    };

    // simulation state
    let mut now: Instant = 0;
    // TODO: multiple current states, with labels
    let mut clock = make_clock(Signal::High, 12_500_000 /* 80MHz */);
    let mut history = vec![];
    while simulate() {
        let (state, mut schedule) = clock.0();
        history.push(("clock", now, state));

        let (step, next) = schedule.pop().unwrap();
        now += step;
        clock = next;
    }

    for (label, ts, state) in history {
        println!("{} t={:9} {:?}", label, ts, state);
    }
}

// some reading:
// - https://en.wikipedia.org/wiki/Discrete-event_simulation
// - https://dev.to/elshize/type-safe-discrete-simulation-in-rust-3n7d

// more reading:
// - https://en.wikipedia.org/wiki/Hardware_description_language
// - https://en.wikipedia.org/wiki/Register-transfer_level
// - https://www.oreilly.com/library/view/introduction-to-digital/9780470900550/chap4-sec003.html
// - https://en.wikipedia.org/wiki/VHDL#Design
// - https://en.wikipedia.org/wiki/Calendar_queue

// lots more reading:
// - https://my.eng.utah.edu/~kstevens/docs/vlsid16.pdf
// - https://www.sciencedirect.com/topics/computer-science/timing-verification

// crates (none of these feel quite right):
// - https://github.com/garro95/desim
// - https://github.com/ndebuhr/sim
// a whole bunch of possibilities from https://lib.rs/simulation
