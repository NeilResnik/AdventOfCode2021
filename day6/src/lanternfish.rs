#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanternFish {
    timer: i32,
}

const DEFAULT_NEW_TIMER: i32 = 8;
const DEFAULT_RESET_TIMER: i32 = 6;
const DEFAULT_REPRO_AMOUNT: usize = 7;

impl LanternFish {
    pub fn default() -> Self {
        LanternFish {
            timer: DEFAULT_NEW_TIMER,
        }
    }

    pub fn new(timer: i32) -> Self {
        LanternFish { timer }
    }

    pub fn maybe_reproduce(&mut self) -> Option<Vec<LanternFish>> {
        self.timer -= 1;
        if self.timer < 0 {
            self.reset();
            Some(vec![LanternFish::default(); DEFAULT_REPRO_AMOUNT])
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.timer = DEFAULT_RESET_TIMER;
    }
}
