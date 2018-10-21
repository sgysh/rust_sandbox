pub struct Memory {
    data: Vec<u8>,
    index: usize,
}

impl Memory {
    pub fn init(s: usize) -> Self {
        Self {
            data: vec![0_u8; s],
            index: 0
        }
    }

    pub fn add(&mut self, n: u8) {
        self.data[self.index] = self.data[self.index].wrapping_add(n);
    }

    pub fn sub(&mut self, n: u8) {
        self.data[self.index] = self.data[self.index].wrapping_sub(n);
    }

    pub fn next(&mut self) {
        self.index += 1;

        let len = self.data.len();
        if self.index >= len {
            self.data.extend(vec![0_u8; len]);
        }
    }

    pub fn prev(&mut self) {
        self.index -= 1;
    }

    pub fn val(&self) -> u8 {
        self.data[self.index]
    }

    pub fn write(&mut self, w: u8) {
        self.data[self.index] = w;
    }
}
