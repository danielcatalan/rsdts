use sdts::{create_system, System, SystemCreator};

fn main() {
    let alpha = 0.9;
    let mut filter1 = create_system!(f32, 1, 2, move |x, y| {
        y[0] = alpha * x[0] + (1.0 - alpha) * y[-1];
    });

    for _ in 0..9 {
        let y = filter1.process(1.0);

        print!("{y}, ");
    }
    println!();
}
