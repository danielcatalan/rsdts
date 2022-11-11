use crate::signal::Signal;
use std::ops::Index;
use std::marker::Copy;

pub struct XSeries<NumType,const XSIZE: usize> {
    signal: Signal<NumType,XSIZE>,
}

impl<NumType:Default+ std::marker::Copy,const XSIZE: usize> XSeries<NumType,XSIZE> {
    pub fn new() -> Self {
        XSeries {
            signal: Signal::new(),
        }
    }

    pub fn push(&mut self, x: NumType) {
        self.signal.push(x);
    }
}

impl<NumType, const XSIZE: usize> Index<i32> for XSeries<NumType, XSIZE >
where
    NumType:Default + Copy
{
    type Output = NumType;
    fn index(&self, inx: i32) -> &Self::Output {
        &self.signal[inx]
    }
}

#[cfg(test)]
mod tests {
    use crate::input_signal::XSeries;

    #[test]
    fn xseries() {
        let mut x_series = XSeries::<f64,3>::new();

        assert_eq!(x_series[0], 0.0);
        assert_eq!(x_series[-1], 0.0);
        assert_eq!(x_series[-2], 0.0);

        x_series.push(1.0);
        assert_eq!(x_series[0], 1.0);
        assert_eq!(x_series[-1], 0.0);
        assert_eq!(x_series[-2], 0.0);

        x_series.push(2.0);
        assert_eq!(x_series[0], 2.0);
        assert_eq!(x_series[-1], 1.0);
        assert_eq!(x_series[-2], 0.0);

        x_series.push(3.0);
        assert_eq!(x_series[0], 3.0);
        assert_eq!(x_series[-1], 2.0);
        assert_eq!(x_series[-2], 1.0);

        x_series.push(4.0);
        assert_eq!(x_series[0], 4.0);
        assert_eq!(x_series[-1], 3.0);
        assert_eq!(x_series[-2], 2.0);
    }
}
