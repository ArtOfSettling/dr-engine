pub(crate) trait PlatformRenderer {
    fn resume(&self);
    fn render(&self);
    fn destroy(&self);
}
