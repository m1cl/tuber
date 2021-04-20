#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum DropDownItems {
    Playlist,
    Song,
    Artist,
}
impl DropDownItems {
    pub const ALL: [DropDownItems; 3] = [
        DropDownItems::Artist,
        DropDownItems::Song,
        DropDownItems::Playlist,
    ];
}

impl Default for DropDownItems {
    fn default() -> DropDownItems {
        DropDownItems::Playlist
    }
}

impl std::fmt::Display for DropDownItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DropDownItems::Artist => "Artist",
                DropDownItems::Playlist => "Playlist",
                DropDownItems::Song => "Song",
            }
        )
    }
}
