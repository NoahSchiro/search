use std::fmt;

pub struct SearchResult {
    pub file: String,
    pub line_num: usize,
    pub content: String
}

impl fmt::Display for SearchResult {

    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}:{}:{}",
            self.file, self.line_num, self.content)
    }
}
