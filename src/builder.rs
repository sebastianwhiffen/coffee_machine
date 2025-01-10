pub trait IBuilder<Built> {
    fn build(&self) -> Built;
    fn add_module(&mut self, module: fn() -> ()) -> &mut Self;
}
