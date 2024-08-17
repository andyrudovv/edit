

mod current_file_module;
pub trait BarModule {
    fn enable(&mut self);
    fn disable(&mut self);
}