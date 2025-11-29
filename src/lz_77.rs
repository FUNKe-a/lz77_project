struct SlidingWindow {
    window: Box<[u8]>,
    search_len: usize,
    lookahead_len: usize,
    pos: usize,
}

impl SlidingWindow {
    pub fn new(window_size: usize, search_len: usize) -> SlidingWindow {
        let lookahead_len = window_size - search_len;
        SlidingWindow {
            window: vec![0u8; window_size].into_boxed_slice(),
            search_len,
            lookahead_len,
            pos: search_len,
        }
    }

    pub fn slide(&mut self, n: usize) {
        if n == 0 { return; }
        let len = self.pos;
        self.window.copy_within(n..len, 0);
        self.pos -= n;
    }

    pub fn refill(&mut self, data: &[u8]) -> usize {
        let available = self.window.len() - self.pos;
        let to_copy = data.len().min(available);

        self.window[self.pos..self.pos + to_copy]
            .copy_from_slice(&data[..to_copy]);
        self.pos += to_copy;

        to_copy
    }
}
