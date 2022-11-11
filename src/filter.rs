use std::marker::PhantomData;

use crate::input_signal::XSeries;
use crate::output_signals::YSeries;

pub trait Filter<NumType> {
    fn filt(&mut self, x: NumType) -> NumType;
}


pub struct DifferenceEquation<NumType,const X: usize, const Y: usize, F>
where
    F: Fn(&XSeries<NumType,X>, &mut YSeries<NumType,Y>),
{
    functor: F,
    xin: XSeries<NumType,X>,
    yout: YSeries<NumType,Y>,
}

impl<NumType:Default+ std::marker::Copy,const X: usize, const Y: usize, F> DifferenceEquation<NumType,X, Y, F>
where
    F: Fn(&XSeries<NumType,X>, &mut YSeries<NumType,Y>),
{
    fn new(function: F) -> Self {
        DifferenceEquation {
            functor: function,
            xin: XSeries::new(),
            yout: YSeries::new(),
        }
    }
}

impl<NumType:Default+ std::marker::Copy,const X: usize, const Y: usize, F> Filter<NumType> for DifferenceEquation<NumType,X, Y, F>
where
    F: Fn(&XSeries<NumType,X>, &mut YSeries<NumType,Y>),
{
    fn filt(&mut self, x: NumType) -> NumType {
        self.xin.push(x);
        self.yout.shift();

        (self.functor)(&self.xin, &mut self.yout);

        return self.yout[0];
    }
}

pub struct FilterCreator<NumType,const X: usize, const Y: usize> {
    phantom: PhantomData<NumType>
}

impl<NumType,const X: usize, const Y: usize> FilterCreator<NumType,X, Y,> 
where
    NumType: Default + std::marker::Copy
{
    pub fn create_filter<F>(function: F) -> DifferenceEquation<NumType,X, Y, F>
    where
        F: Fn(&XSeries<NumType,X>, &mut YSeries<NumType,Y>),
    {
        DifferenceEquation::new(function)
    }
}

#[macro_export] macro_rules! create_filter {
    ($XS:literal, $YS:literal, $e:expr) => {
        FilterCreator::<f64,$XS,$YS>::create_filter($e)
    };
}
