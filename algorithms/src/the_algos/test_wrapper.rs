pub struct TestHandler<T, W> {
    pub input: T,
    pub expected: W,
}

impl<T, W> TestHandler<T, W> {
    pub fn new(input: T, expected: W) -> Self {
        TestHandler { input, expected }
    }
}
