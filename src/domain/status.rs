#[derive(Clone, Copy, Debug)]
pub enum Status {
    NotStarted,
    Playing,
    Completed,
    Abandoned,
}
