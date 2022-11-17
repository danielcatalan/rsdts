#[cfg(test)]
mod integration_test {
    use crate::{create_system, SystemCreator};

    #[test]
    fn straight_through() {
        let mut filter = create_system!(1, 1, |x, y| {
            y[0] = x[0];
        });

        let x_in: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mut y_out: [f64; 6] = [0.0; 6];

        for i in 0..6 {
            y_out[i] = filter.process(x_in[i]);
        }

        assert_eq!(y_out, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    }

    #[test]
    pub fn delay() {
        let mut filt1 = create_system!(2, 1, |x, y| {
            y[0] = x[-1];
        });

        let xin = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = [0.0, 1.0, 2.0, 3.0, 4.0];
        let mut yout = [0.0, 0.0, 0.0, 0.0, 0.0];

        for i in 0..5 {
            yout[i] = filt1.process(xin[i]);
        }

        assert_eq!(yout, expected);
    }
}
