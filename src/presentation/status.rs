use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum Status {
    NotStarted,
    Playing,
    Completed,
    Abandoned,
}
