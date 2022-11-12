use std::{
    marker::Copy,
    ops::{Index, IndexMut},
};

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

    pub fn push(&mut self, x: NumType) {
        self.zero_index = (self.zero_index + 1) % SIZE;

        self._signal[self.zero_index] = x;
    }

    pub fn shift(&mut self) {
        let zero_index = &self.zero_index;
        self.zero_index = (zero_index + 1) % SIZE;
    }
}

impl<NumType, const SIZE: usize> Index<i32> for Signal<NumType, SIZE>
where
    NumType: Default + Copy,
{
    type Output = NumType;

    fn index(&self, inx: i32) -> &Self::Output {
        let i: usize = self.get_corrected_index(inx);
        &self._signal[i]
    }
}

impl<NumType, const SIZE: usize> IndexMut<i32> for Signal<NumType, SIZE>
where
    NumType: Default + Copy,
{
    fn index_mut(&mut self, inx: i32) -> &mut Self::Output {
        let i: usize = self.get_corrected_index(inx);
        &mut self._signal[i]
    }
}

#[cfg(test)]
mod test {
    use super::Signal;

    #[test]
    fn new() {
        let signal = Signal::<f64, 4>::new();

        assert_eq!(signal._signal.len(), 4);
        assert_eq!(signal._signal, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(signal.zero_index, 3);
    }

    #[test]
    fn push() {
        let mut signal = Signal::<f64, 4>::new();

        signal.push(1.0);
        assert_eq!(signal._signal, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(signal.zero_index, 0);
        signal.push(2.0);
        assert_eq!(signal._signal, [1.0, 2.0, 0.0, 0.0]);
        assert_eq!(signal.zero_index, 1);
        signal.push(3.0);
        assert_eq!(signal._signal, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(signal.zero_index, 2);
        signal.push(4.0);
        assert_eq!(signal._signal, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(signal.zero_index, 3);
        signal.push(5.0);
        assert_eq!(signal._signal, [5.0, 2.0, 3.0, 4.0]);
        assert_eq!(signal.zero_index, 0);
    }

    #[test]
    fn shift() {
        let mut signal = Signal::<f64, 4>::new();

        signal.shift();
        assert_eq!(signal.zero_index, 0);
        signal.shift();
        assert_eq!(signal.zero_index, 1);
        signal.shift();
        assert_eq!(signal.zero_index, 2);
        signal.shift();
        assert_eq!(signal.zero_index, 3);
        signal.shift();
        assert_eq!(signal.zero_index, 0);
    }

    #[test]
    fn indexing() {
        let signal = Signal {
            _signal: [4.0, 3.0, 2.0, 1.0],
            zero_index: 3,
        };

        assert_eq!(signal[0], 1.0);
        assert_eq!(signal[-1], 2.0);
        assert_eq!(signal[-2], 3.0);
    }
}
