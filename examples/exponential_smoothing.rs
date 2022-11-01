
use simple_discrete_time_system::filter::Filter;


fn main(){
    let alpha = 0.1;
    {
        let mut filter1 = Filter::<1,2>::create_filter(|x,y|{
            y[0] = alpha * x[0] + (1.0-alpha) * y[-1];
        });

        for _ in 0..10{
            let y = filter1.filt(1.0);

            print!("{y:.3}, ");
        }
        println!();
    }


}