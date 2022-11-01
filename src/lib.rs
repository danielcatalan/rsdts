mod filter;
mod signal;
mod input_signal;
mod output_signals;

#[cfg(test)]
mod tests {

    use crate::filter::Filter;

    #[test]
    fn in_out() {

        let mut filter = Filter::<2, 2>::create_filter(|x, y| {
            y[0] = x[0];
        });

        let x_in: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut y_out: [f64; 6] = [0.0; 6];

        for i in 0..6 {
            y_out[i] = filter.filt(x_in[i]);
        }

        assert_eq!(y_out, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    }

    #[test]
    fn delay() {

        let mut filter = Filter::<2, 2>::create_filter(|x, y| {
            y[0] = x[-1];
        });

        let x_in: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut y_out: [f64; 6] = [0.0; 6];

        for i in 0..6 {
            y_out[i] = filter.filt(x_in[i]);
        }

        assert_eq!(y_out, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0])
    }
}
