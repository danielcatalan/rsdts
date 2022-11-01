pub struct DifferenceEquation<const X: usize, const Y: usize, F> {
    pub functor: F,
    xin: [f64; X],
    yout: [f64; Y],
}

impl<const X: usize, const Y: usize, F> DifferenceEquation<X, Y, F> {
    fn new(function: F) -> Self {
        DifferenceEquation {
            functor: function,
            xin: [0.0; X],
            yout: [0.0; Y],
        }
    }

    pub fn filt(&self, x: f64) -> f64 {
        // self.xin[0] = x;
        // self.functor(&self.xin, &mut self.yout);

        // return self.yout[0];
        0.0
    }
}

pub struct Filter<const X: usize, const Y: usize> {}

impl<const X: usize, const Y: usize>  Filter<X, Y> {
    pub fn create_filter<F: >(function: F) -> DifferenceEquation<X, Y, F> {
        DifferenceEquation::new(function)
    }
}
