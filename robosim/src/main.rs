use std::{fmt::Debug, vec};

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

// #[derive(Debug, Clone)]
// pub struct Bus {
//     lines: Vec<Signal>,
// }

// #[derive(Debug, Clone)]
// pub enum State {
//     Signal(Signal),
//     Bus(Bus),
// }

// fugit::Instant can only do u32, u64. Do we need signed? idk
// type Instant = fugit::Instant<i64, 1, 1>;

type Instant = i64; // what, femtoseconds?
type Duration = i64; // what, femtoseconds?
type Step = i64; // what, femtoseconds?

// see https://stackoverflow.com/questions/27886474/recursive-function-type
// TODO: generator? https://doc.rust-lang.org/beta/unstable-book/language-features/generators.html
pub struct StateFn {
    f: Box<dyn FnOnce() -> (Signal, Vec<(Step, StateFn)>)>,
}

// TODO: would something like this be better?
#[allow(dead_code)]
pub struct _State {
    state: Signal,
    next: Box<dyn FnOnce() -> Vec<(Step, _State)>>,
}

// TODO: or this?
#[allow(dead_code)]
pub struct PinState {
    pub state: Signal,
    pub next: Box<dyn FnOnce() -> Vec<(Step, PinState)>>,
}

pub type Pins = Vec<(String, PinState)>;

#[allow(dead_code)]
fn make_clock2(initial_state: Signal, pulse_width: Duration) -> Pins {
    fn clock(state: Signal, pulse_width: Duration) -> Box<dyn (FnOnce() -> Vec<(Step, PinState)>)> {
        Box::new(move || {
            return vec![(
                pulse_width,
                PinState {
                    state: state,
                    next: Box::new(clock(state.invert(), pulse_width)),
                },
            )];
        })
    }
    return vec![(
        "clock".to_string(),
        PinState {
            state: initial_state,
            next: clock(initial_state.invert(), pulse_width),
        },
    )];
}

// "wire"s the output into the given inputs, forming a node in the wiring DAG
// like clojure's `->` form
// TODO: circuits, definitionally, are not DAGs?
pub fn wire1(
    output: StateFn,
    mut inputs: Vec<Box<dyn FnMut(Signal) -> Vec<(Step, StateFn)>>>,
) -> StateFn {
    StateFn {
        f: Box::new(move || -> (Signal, Vec<(Step, StateFn)>) {
            let (signal, mut steps) = (output.f)();

            let downstream: Vec<_> = inputs.iter_mut().map(|f| f(signal)).flatten().collect();

            // let mut rewired: Vec<_> = steps
            //     .into_iter()
            //     .map(move |(step, f)| (step, wire1(f, inputs)))
            //     .collect();
            let (delay, clock) = steps.pop().expect("a single clock element");

            let mut rewired = vec![(delay, wire1(clock, inputs))];

            rewired.extend(downstream);
            (signal, rewired)
        }),
    }
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
        let clock = move || -> _ {
            (
                initial_state,
                vec![(pulse_width, make_clock(initial_state.invert(), pulse_width))],
            )
        };
        return StateFn { f: Box::new(clock) };
    }

    fn make_inverter() -> Box<dyn FnMut(Signal) -> Vec<(Step, StateFn)>> {
        Box::new(move |input: Signal| -> _ {
            vec![(
                6_000_000, // 6ns gate delay
                StateFn {
                    f: Box::new(move || -> (Signal, Vec<(Step, StateFn)>) {
                        (input.invert(), vec![])
                    }),
                },
            )]
        })
    }

    let mut total_steps = 6;
    let mut simulate = move || -> bool {
        total_steps -= 1;
        total_steps > 0
    };

    // simulation state
    let mut now: Instant = 0;
    // TODO: multiple current states, with labels
    // TODO: determinism within a run & across multiple runs (i.e. seeded random)?
    let clock = make_clock(Signal::High, 12_500_000 /* 80MHz */);

    // TODO: what's the inverter's initial state?
    let mut root = wire1(clock, vec![make_inverter()]);

    let mut history = vec![];
    while simulate() {
        let (state, mut schedule) = (root.f)();
        history.push(("clock", now, state));

        let (inv_step, next) = schedule.pop().unwrap();
        let (state, _) = (next.f)();
        history.push(("inv", now + inv_step, state));

        let (clock_step, next) = schedule.pop().unwrap();
        now += clock_step;
        root = next;
    }

    for (label, ts, state) in history {
        println!("t={:9} {:8} {:?}", ts, label, state);
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
