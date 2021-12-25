#[derive(Debug)]
pub enum ImporterError {
    OsmPbfError(osmpbf::Error),
}

impl From<osmpbf::Error> for ImporterError {
    fn from(error: osmpbf::Error) -> Self {
        ImporterError::OsmPbfError(error)
    }
}
