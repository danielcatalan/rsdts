use crate::input_signal::InputSignal;
use crate::output_signals::OutputSignal;
use core::marker::PhantomData;

pub trait System<NumType> {
    fn process(&mut self, x: NumType) -> NumType;
}

pub struct DifferenceEquation<NumType, const X: usize, const Y: usize, F>
where
    F: Fn(&InputSignal<NumType, X>, &mut OutputSignal<NumType, Y>),
{
    func: F,
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
            func: function,
            xin: InputSignal::new(),
            yout: OutputSignal::new(),
        }
    }

    pub fn process(&mut self, x: NumType) -> NumType {
        self.xin.push(x);
        self.yout.shift();

        (self.func)(&self.xin, &mut self.yout);

        self.yout[0]
    }
}

impl<NumType, const X: usize, const Y: usize, F> System<NumType>
    for DifferenceEquation<NumType, X, Y, F>
where
    NumType: Default + Copy,
    F: Fn(&InputSignal<NumType, X>, &mut OutputSignal<NumType, Y>),
{
    fn process(&mut self, x: NumType) -> NumType {
        self.process(x)
    }
}

pub struct SystemCreator<NumType, const X: usize, const Y: usize> {
    phantom: PhantomData<NumType>,
}

impl<NumType, const X: usize, const Y: usize> SystemCreator<NumType, X, Y>
where
    NumType: Default + Copy,
{
    pub fn create_system<F>(function: F) -> DifferenceEquation<NumType, X, Y, F>
    where
        F: Fn(&InputSignal<NumType, X>, &mut OutputSignal<NumType, Y>),
    {
        DifferenceEquation::new(function)
    }
}

/// Macro to create Discrete Time System.
///
/// # Examples
/// ```
/// use sdts::create_system;
///
/// // Create Delay System
/// let mut filter = create_system!(2,1, |x,y|{
///     y[0] = x[-1];
/// });
/// ```
///
#[macro_export]
macro_rules! create_system {
    ($XS:literal, $YS:literal, $e:expr) => {
        $crate::SystemCreator::<f64, $XS, $YS>::create_system($e)
    };

    ($T:ty, $XS:literal, $YS:literal, $e:expr) => {
        $crate::SystemCreator::<$T, $XS, $YS>::create_system($e)
    };
}
