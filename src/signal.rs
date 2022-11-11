use std::marker::Copy;

/// Signal
///
/// An array-like FIFO container that deals with elements in present and past terms
///
pub struct Signal<NumType, const SIZE: usize> {
    _signal: [NumType; SIZE],
    zero_index: usize,
}

impl<NumType, const SIZE: usize> Signal<NumType, SIZE>
where
    NumType: Default + Copy,
{
    pub fn new() -> Self {
        Signal {
            _signal: [NumType::default(); SIZE],
            zero_index: (SIZE - 1),
        }
    }

    fn get_corrected_index(&self, n: i32) -> usize {
        if n < 0 {
            let mut some = (SIZE + self.zero_index) as i32;
            some = some + n;
            return (some as usize) % SIZE;
        }
        return self.zero_index;
    }

    pub fn get_index(&self, inx: i32) -> &NumType {
        let i: usize = self.get_corrected_index(inx);
        &self._signal[i]
    }

    pub fn get_index_mut(&mut self, inx: i32) -> &mut NumType {
        let i: usize = self.get_corrected_index(inx);
        &mut self._signal[i]
    }

    pub fn push(&mut self, x: NumType) {
        self.zero_index = (self.zero_index + 1) % SIZE;

        self._signal[self.zero_index] = x;
    }

    pub fn shift(&mut self) {
        let zero_index = &self.zero_index;
        self.zero_index = (zero_index + 1) % SIZE;
    }
}

#[cfg(test)]
mod test {
    use super::Signal;

    #[test]
    fn new() {
        let series = Signal::<f64, 4>::new();

        assert_eq!(series._signal.len(), 4);
        assert_eq!(series._signal, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(series.zero_index, 3);
    }

    #[test]
    fn push() {
        let mut series = Signal::<f64, 4>::new();

        series.push(1.0);
        assert_eq!(series._signal, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(series.zero_index, 0);
        series.push(2.0);
        assert_eq!(series._signal, [1.0, 2.0, 0.0, 0.0]);
        assert_eq!(series.zero_index, 1);
        series.push(3.0);
        assert_eq!(series._signal, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(series.zero_index, 2);
        series.push(4.0);
        assert_eq!(series._signal, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(series.zero_index, 3);
        series.push(5.0);
        assert_eq!(series._signal, [5.0, 2.0, 3.0, 4.0]);
        assert_eq!(series.zero_index, 0);
    }

    #[test]
    fn shift() {
        let mut series = Signal::<f64, 4>::new();

        series.shift();
        assert_eq!(series.zero_index, 0);
        series.shift();
        assert_eq!(series.zero_index, 1);
        series.shift();
        assert_eq!(series.zero_index, 2);
        series.shift();
        assert_eq!(series.zero_index, 3);
        series.shift();
        assert_eq!(series.zero_index, 0);
    }
}
