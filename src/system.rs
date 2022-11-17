use crate::input_signal::InputSignal;
use crate::output_signals::OutputSignal;
use std::marker::PhantomData;

pub trait System<NumType> {
    fn process(&mut self, x: NumType) -> NumType;
}

pub struct DifferenceEquation<NumType, const X: usize, const Y: usize, F>
where
    F: Fn(&InputSignal<NumType, X>, &mut OutputSignal<NumType, Y>),
{
    functor: F,
    xin: InputSignal<NumType, X>,
    yout: OutputSignal<NumType, Y>,
}

impl<NumType, const X: usize, const Y: usize, F> DifferenceEquation<NumType, X, Y, F>
where
    NumType: Default + Copy,
    F: Fn(&InputSignal<NumType, X>, &mut OutputSignal<NumType, Y>),
{
    fn new(function: F) -> Self {
        DifferenceEquation {
            functor: function,
            xin: InputSignal::new(),
            yout: OutputSignal::new(),
        }
    }
}

impl<NumType, const X: usize, const Y: usize, F> System<NumType>
    for DifferenceEquation<NumType, X, Y, F>
where
    NumType: Default + std::marker::Copy,
    F: Fn(&InputSignal<NumType, X>, &mut OutputSignal<NumType, Y>),
{
    fn process(&mut self, x: NumType) -> NumType {
        self.xin.push(x);
        self.yout.shift();

        (self.functor)(&self.xin, &mut self.yout);

        return self.yout[0];
    }
}

pub struct FilterCreator<NumType, const X: usize, const Y: usize> {
    phantom: PhantomData<NumType>,
}

impl<NumType, const X: usize, const Y: usize> FilterCreator<NumType, X, Y>
where
    NumType: Default + Copy,
{
    pub fn create_filter<F>(function: F) -> DifferenceEquation<NumType, X, Y, F>
    where
        F: Fn(&InputSignal<NumType, X>, &mut OutputSignal<NumType, Y>),
    {
        DifferenceEquation::new(function)
    }
}

#[macro_export]
macro_rules! create_filter {
    ($XS:literal, $YS:literal, $e:expr) => {
        FilterCreator::<f64, $XS, $YS>::create_filter($e)
    };

    ($T:ty, $XS:literal, $YS:literal, $e:expr) => {
        FilterCreator::<$T, $XS, $YS>::create_filter($e)
    };
    
}
