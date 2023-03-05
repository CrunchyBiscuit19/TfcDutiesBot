#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    PerformAction,
    HandleParadeState,
    GetName,
    GetMonth {name: String},
}