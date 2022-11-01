use crate::signal::Signal;
use std::ops::{Index,IndexMut};


pub struct YSeries<const YSIZE: usize> {
    signal: Signal<YSIZE>,
}

impl<const YSIZE: usize> YSeries<YSIZE> {

    pub fn new() -> Self{
        YSeries {
            signal: Signal::new(),
        }
    }

    pub fn shift(&mut self){
        self.signal.shift();
    }
}

impl<const YSIZE: usize> Index<i32> for YSeries<{ YSIZE }> {
    type Output = f64;
    fn index(&self, inx: i32) -> &Self::Output {
        self.signal.get_index(inx)
    }
}

impl<const YSIZE: usize> IndexMut<i32> for YSeries<{ YSIZE }> {
    fn index_mut(&mut self, inx: i32) -> &mut Self::Output {
        self.signal.get_index_mut(inx)
    }


}

#[cfg(test)]
mod tests {
    use crate::output_signals::YSeries;

    #[test]
    fn yseries(){
        let mut y_series = YSeries::<3>::new();

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