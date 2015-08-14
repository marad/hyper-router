extern crate regex;
use self::regex::Regex;

/// Represents a path in HTTP sense (starting from `/`)
#[derive(Debug)]
pub struct Path {
    pub matcher: Regex
}

impl Path {
    /// Creates a new path.
    ///
    /// This method accepts regular expressions so you can 
    /// write something like this:
    ///
    /// ```rust
    /// Path::new(r"/person/\d+")
    /// ```
    ///
    /// Note that you don't have to match beggining and end of the 
    /// path using `^` and `$` - those are inserted for you automatically.
    pub fn new(path: &str) -> Path {
        let mut regex = "^".to_string();
        regex.push_str(path);
        regex.push_str("$");
        Path { matcher: Regex::new(&regex).unwrap() }
    }
}

