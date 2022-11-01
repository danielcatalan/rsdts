use crate::signal::Signal;
use std::ops::Index;

pub struct XSeries<const XSIZE: usize> {
    signal: Signal<XSIZE>,
}

impl<const XSIZE: usize> XSeries<XSIZE> {
    pub fn new() -> Self {
        XSeries {
            signal: Signal::new(),
        }
    }

    pub fn push(&mut self, x: f64) {
        self.signal.push(x);
    }
}

impl<const XSIZE: usize> Index<i32> for XSeries<{ XSIZE }> {
    type Output = f64;
    fn index(&self, inx: i32) -> &Self::Output {
        self.signal.get_index(inx)
    }
}

#[cfg(test)]
mod tests {
    use crate::input_signal::XSeries;

    #[test]
    fn xseries() {
        let mut x_series = XSeries::<3>::new();

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
