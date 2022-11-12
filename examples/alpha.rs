use sdts::{create_filter, Filter, FilterCreator};

fn main() {
    let alpha = 0.9;
    let mut filter1 = create_filter!(1, 2, move |x, y| {
        y[0] = alpha * x[0] + (1.0 - alpha) * y[-1];
    });

    for _ in 0..6 {
        let y = filter1.filt(1.0);

        print!("{y}, ");
    }
    println!();
}
