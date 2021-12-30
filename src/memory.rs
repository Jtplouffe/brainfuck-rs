pub(crate) struct Memory {
    dp: usize,
    data: Vec<i32>,
}

impl Memory {
    pub(crate) fn new() -> Self {
        Memory {
            dp: 0,
            data: vec![0; 30000],
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
