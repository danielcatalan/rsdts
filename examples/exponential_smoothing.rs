
use sdts::filter::{FilterCreator, Filter};


fn foo() -> Box<dyn Filter>{
    let alpha = 0.9;

    let filter1 = Box::new(FilterCreator::<1,2>::create_filter(move |x,y|{
        y[0] = alpha * x[0] + (1.0-alpha) * y[-1];
    }));
    return filter1;
}

fn main(){

    let mut filter1 = foo(); 

    for _ in 0..6{
        let y = filter1.filt(1.0);

        print!("{y}, ");
    }
    println!();
}