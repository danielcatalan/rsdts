
pub(crate) struct Signal<const SIZE: usize>{
    pub _signal: [f64; SIZE],
    pub zero_index: usize
}

impl<const SIZE: usize> Signal<SIZE> {
   pub fn new() -> Self {
        Signal{
            _signal: [0.0 ; SIZE],
            zero_index: (SIZE - 1)
        }
    }

    fn get_corrected_index(&self, n: i32) ->usize {
        if n < 0
        {

            let mut some = (SIZE  + self.zero_index) as i32;
            some = some + n;
            return (some as usize ) % SIZE;
        }
        return self.zero_index;
    }


    pub fn get_index(&self, inx: i32) -> &f64 {
        let i: usize = self.get_corrected_index(inx) ;
        &self._signal[i]
    }

    pub fn get_index_mut(&mut self, inx: i32) -> &mut f64 {
        let i: usize = self.get_corrected_index(inx) ;
        &mut self._signal[i]
    }

    pub fn push(&mut self, x: f64) 
    {
        self.zero_index = (self.zero_index + 1) % SIZE;

        self._signal[self.zero_index] = x;
    }

    pub fn shift(&mut self)
    {
        let zero_index  = &self.zero_index;
        self.zero_index = (zero_index + 1) % SIZE;
    }

}


