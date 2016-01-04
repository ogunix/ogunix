pub trait Module {
    fn init(&self);
    fn exit(&self);
}
