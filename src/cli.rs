#[derive(clap::Parser)]
pub struct Cli {
    pub arg: Option<StoryType>,
}

#[derive(Clone, clap::ValueEnum)]
pub enum StoryType {
    New,
    Top,
    Best,

    Ask,
    Show,
    Job,
}
