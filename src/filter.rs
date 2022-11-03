use crate::input_signal::XSeries;
use crate::output_signals::YSeries;

pub trait DiffEq{

    fn filt(&mut self, x: f64) -> f64;
}

pub struct DifferenceEquation<const X: usize, const Y: usize, F: Fn(&XSeries<X>,&mut YSeries<Y>)> {
    functor: F,
    xin: XSeries<X>,
    yout: YSeries<Y>,
}

impl<const X: usize, const Y: usize, F: Fn(&XSeries<X>,&mut YSeries<Y>)> DifferenceEquation<X,Y,F> {
    fn new(function: F) -> Self {
        DifferenceEquation {
            functor: function,
            xin: XSeries::new(),
            yout: YSeries::new(),
        }
    }
}
impl<const X: usize, const Y: usize, F: Fn(&XSeries<X>,&mut YSeries<Y>)> DiffEq for DifferenceEquation<X,Y,F>{

    fn filt(&mut self, x: f64) -> f64 {
        self.xin.push(x);
        self.yout.shift();

        (self.functor)(&self.xin, &mut self.yout);

        return self.yout[0];
    }
}

pub struct Filter<const X: usize, const Y: usize> {}

impl<const X: usize, const Y: usize>  Filter<X, Y> {
    pub fn create_filter<F: Fn(&XSeries<X>,&mut YSeries<Y>)>(function: F) -> DifferenceEquation<X, Y, F> {
        DifferenceEquation::new(function)
    }
}
