mod filter;

#[cfg(test)]
mod tests {

    use crate::filter::Filter;

    #[test]
    fn basic_filter() {

        let filter = Filter::<2, 1>::create_filter(|x, y| {
            y[0] = x[-1];
        });

        let x_in: [f64; 6] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let mut y_in: [f64; 6] = [0.0; 6];

        for i in 0..6 {
            y_in[i] = filter.filt(x_in[i]);
        }
    }
}
