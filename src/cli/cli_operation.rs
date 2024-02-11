use super::{ExtractOptions, ListOptions};

pub enum CliOperation {
    List(ListOptions),
    Extract(ExtractOptions),
    Help,
}
