pub(crate) struct Memory<const N: usize = 30000> {
    dp: usize,
    data: [i32; N],
}

impl<const N: usize> Memory<N> {
    pub(crate) fn new() -> Self {
        Memory {
            dp: 0,
            data: [0; N],
        }
    }

    pub(crate) fn increment_dp(&mut self) {
        self.dp += 1;
    }

    pub(crate) fn decrement_dp(&mut self) {
        self.dp -= 1;
    }

    pub(crate) fn increment(&mut self) {
        self.data[self.dp] += 1;
    }

    pub(crate) fn decrement(&mut self) {
        self.data[self.dp] -= 1;
    }

    pub(crate) fn value(&self) -> i32 {
        self.data[self.dp]
    }

    pub(crate) fn set_value(&mut self, value: i32) {
        self.data[self.dp] = value;
    }
}
