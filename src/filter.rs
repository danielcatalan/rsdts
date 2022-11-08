use crate::input_signal::XSeries;
use crate::output_signals::YSeries;

pub trait Filter {
    fn filt(&mut self, x: f64) -> f64;
}


pub struct DifferenceEquation<const X: usize, const Y: usize, F>
where
    F: Fn(&XSeries<X>, &mut YSeries<Y>),
{
    functor: F,
    xin: XSeries<X>,
    yout: YSeries<Y>,
}

impl<const X: usize, const Y: usize, F> DifferenceEquation<X, Y, F>
where
    F: Fn(&XSeries<X>, &mut YSeries<Y>),
{
    fn new(function: F) -> Self {
        DifferenceEquation {
            functor: function,
            xin: XSeries::new(),
            yout: YSeries::new(),
        }
    }
}

impl<const X: usize, const Y: usize, F> Filter for DifferenceEquation<X, Y, F>
where
    F: Fn(&XSeries<X>, &mut YSeries<Y>),
{
    fn filt(&mut self, x: f64) -> f64 {
        self.xin.push(x);
        self.yout.shift();

        (self.functor)(&self.xin, &mut self.yout);

        return self.yout[0];
    }
}

pub struct FilterCreator<const X: usize, const Y: usize> {}

impl<const X: usize, const Y: usize> FilterCreator<X, Y> {
    pub fn create_filter<F>(function: F) -> DifferenceEquation<X, Y, F>
    where
        F: Fn(&XSeries<X>, &mut YSeries<Y>),
    {
        DifferenceEquation::new(function)
    }
}

#[macro_export] macro_rules! create_filter {
    ($XS:literal, $YS:literal, $e:expr) => {
        FilterCreator::<$XS,$YS>::create_filter($e)
    };
}
