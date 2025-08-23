pub trait Step {
    /// Advance the component by the given number of CPU cycles.
    fn step(&mut self, cycles: u8);
}