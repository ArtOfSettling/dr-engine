pub trait EventHandler {
    fn resume(&self);
    fn render(&self);
    fn destroy(&self);
}
