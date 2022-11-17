use crate::signal::Signal;
// use std::ops::{Index, IndexMut};
use core::ops::{Index, IndexMut};

pub struct OutputSignal<NumType, const YSIZE: usize> {
    signal: Signal<NumType, YSIZE>,
}

impl<NumType, const YSIZE: usize> OutputSignal<NumType, YSIZE>
where
    NumType: Default + Copy,
{
    pub fn new() -> Self {
        OutputSignal {
            signal: Signal::new(),
        }
    }

    pub fn shift(&mut self) {
        self.signal.shift();
    }
}

impl<NumType, const YSIZE: usize> Index<i32> for OutputSignal<NumType, YSIZE>
where
    NumType: Default + Copy,
{
    type Output = NumType;
    fn index(&self, inx: i32) -> &Self::Output {
        &self.signal[inx]
    }
}

impl<NumType, const YSIZE: usize> IndexMut<i32> for OutputSignal<NumType, YSIZE>
where
    NumType: Default + Copy,
{
    fn index_mut(&mut self, inx: i32) -> &mut Self::Output {
        &mut self.signal[inx]
    }
}

#[cfg(test)]
mod tests {
    use crate::output_signals::OutputSignal;

    #[test]
    fn yseries() {
        let mut y_series = OutputSignal::<f64, 3>::new();

        assert_eq!(y_series[0], 0.0);
        assert_eq!(y_series[-1], 0.0);
        assert_eq!(y_series[-2], 0.0);

        y_series[0] = 1.0;
        assert_eq!(y_series[0], 1.0);
        assert_eq!(y_series[-1], 0.0);
        assert_eq!(y_series[-2], 0.0);

        y_series.shift();
        y_series[0] = 2.0;
        assert_eq!(y_series[0], 2.0);
        assert_eq!(y_series[-1], 1.0);
        assert_eq!(y_series[-2], 0.0);
    }
}
