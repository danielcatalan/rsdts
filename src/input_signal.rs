use crate::signal::Signal;
use core::ops::Index;

pub struct InputSignal<NumType, const XSIZE: usize> {
    signal: Signal<NumType, XSIZE>,
}

impl<NumType, const XSIZE: usize> InputSignal<NumType, XSIZE>
where
    NumType: Default + Copy,
{
    pub fn new() -> Self {
        InputSignal {
            signal: Signal::new(),
        }
    }

    pub fn push(&mut self, x: NumType) {
        self.signal.push(x);
    }
}

impl<NumType, const XSIZE: usize> Default for InputSignal<NumType, XSIZE>
where
    NumType: Default + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<NumType, const XSIZE: usize> Index<i32> for InputSignal<NumType, XSIZE>
where
    NumType: Default + Copy,
{
    type Output = NumType;
    fn index(&self, inx: i32) -> &Self::Output {
        &self.signal[inx]
    }
}

#[cfg(test)]
mod tests {
    use crate::input_signal::InputSignal;

    #[test]
    fn xseries() {
        let mut x_signal = InputSignal::<f64, 3>::new();

        assert_eq!(x_signal[0], 0.0);
        assert_eq!(x_signal[-1], 0.0);
        assert_eq!(x_signal[-2], 0.0);

        x_signal.push(1.0);
        assert_eq!(x_signal[0], 1.0);
        assert_eq!(x_signal[-1], 0.0);
        assert_eq!(x_signal[-2], 0.0);

        x_signal.push(2.0);
        assert_eq!(x_signal[0], 2.0);
        assert_eq!(x_signal[-1], 1.0);
        assert_eq!(x_signal[-2], 0.0);

        x_signal.push(3.0);
        assert_eq!(x_signal[0], 3.0);
        assert_eq!(x_signal[-1], 2.0);
        assert_eq!(x_signal[-2], 1.0);

        x_signal.push(4.0);
        assert_eq!(x_signal[0], 4.0);
        assert_eq!(x_signal[-1], 3.0);
        assert_eq!(x_signal[-2], 2.0);
    }
}
