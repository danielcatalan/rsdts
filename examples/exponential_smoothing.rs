
use simple_discrete_time_system::filter::{Filter,DiffEq};


fn foo() -> Box<dyn DiffEq>{
    let alpha = 0.1;

    let filter1 = Box::new(Filter::<1,2>::create_filter(move |x,y|{
        y[0] = alpha * x[0] + (1.0-alpha) * y[-1];
    }));
    return filter1;
}

fn main(){

    let mut filter1 = foo(); 

    for _ in 0..10{
        let y = filter1.filt(1.0);

        print!("{y:.3}, ");
    }
    println!();
}