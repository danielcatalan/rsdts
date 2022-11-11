
use sdts::{Filter,create_filter,FilterCreator};

fn exp_smooth(a:f64) -> Box<dyn Filter>{
    Box::new(create_filter!(1,2, move |x,y| {
        y[0] = a * x[0] + (1.0-a) * y[-1];
    }))
}

fn main(){
    
    let mut filter1 = exp_smooth(0.5); 

    for _ in 0..6{
        let y = filter1.filt(1.0);

        print!("{y}, ");
    }
    println!();
}