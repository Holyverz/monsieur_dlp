#[derive(Clone, Debug, PartialEq)]
pub struct Song {
    pub url: String,
    pub artist: String,
    pub name: String,
}

impl Song {
    pub fn new(url: String, artist: String, name: String) -> Self {
        Self {
            url,
            artist,
            name,
        }
    }
}
impl std::fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}|{}|{}", self.url, self.artist, self.name)
    }
}