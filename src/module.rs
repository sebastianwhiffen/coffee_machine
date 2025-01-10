pub trait IModule<TBuilt, TBuilder> {
    fn run(&self);
    fn create_builder() -> TBuilder;
}
