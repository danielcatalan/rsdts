use sdts::{create_system, SystemCreator};

fn main() {
    let alpha = 0.9;
    let mut filter1 = create_system!(f32, 1, 2, move |x, y| {
        y[0] = alpha * x[0] + (1.0 - alpha) * y[-1];
    });

    let x = [1.0;9];
    let mut y = [0.0; 9];
    
    for i in 0..9 {
        y[i] = filter1.process(x[i]);

        print!("{}, ", y[i]);
    }
    println!();
}
