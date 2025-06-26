///! The catalog of search modes available for searching tweets, profiles, etc.
#[derive(Debug, Clone, Copy)]
pub enum SearchMode {
    Top,
    Latest,
    Photos,
    Videos,
    Users,
}
