use crate::parameters::RouteParameters;
use std::collections::HashMap;

/// Represents a path in HTTP sense (starting from `/`)
/// This path is internal to the crate, and encapsulates the path matching
/// logic of a route.
#[derive(Debug)]
pub(crate) enum Path {
    Static(String),
    Parametric(Vec<String>),
}

impl Path {
    pub fn new(path: &str) -> Path {
        if is_parametric_path(path) {
            Path::Parametric(path.split('/').map(|s| s.to_owned()).collect())
        } else {
            Path::Static(path.to_owned())
        }
    }

    pub fn matches(&self, other_path: &str) -> Option<RouteParameters> {
        match self {
            Path::Static(me) => {
                if me == other_path {
                    Some(RouteParameters::none())
                } else {
                    None
                }
            }
            Path::Parametric(self_path) => {
                let mut params = HashMap::new();
                let mut self_segments = self_path.iter();
                let mut other_segments = other_path.split('/');

                loop {
                    let self_seg = self_segments.next();
                    let other_seg = other_segments.next();

                    match (self_seg, other_seg) {
                        // We have two segments to compare.
                        (Some(left), Some(right)) => {
                            let mut self_chars = left.chars();
                            let first_self_char = self_chars.nth(0);
                            match first_self_char {
                                Some(ch) if ch == ':' => {
                                    let key: String = self_chars.collect();
                                    params.insert(key, right.to_string());
                                }
                                _ => {
                                    if left != right {
                                        return None;
                                    }
                                }
                            };
                        }

                        // We're out of segments to compare, so it's a match.
                        (None, None) => {
                            return Some(RouteParameters::new(params));
                        }

                        // We have 1 Some and 1 None, meaning the route can't be a match
                        _ => {
                            return None;
                        }
                    }
                }
            }
        }
    }
}

fn is_parametric_path(path: &str) -> bool {
    for ch in path.chars() {
        if ch == ':' {
            return true;
        }
    }
    return false;
}

// We only impl PartialEq for Path when we're compiling in test configuration so
// we can use assert_eq!(path_1, path_2)
#[cfg(test)]
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Path::Static(self_str), Path::Static(other_str)) => {
                return self_str == other_str;
            }
            (Path::Parametric(self_vec), Path::Parametric(other_vec)) => {
                if self_vec.len() != other_vec.len() {
                    return false;
                }
                for (a, b) in self_vec.iter().zip(other_vec.iter()) {
                    if a != b {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_parametric_path_on_static_paths() {
        assert_eq!(is_parametric_path("/foo"), false);
        assert_eq!(is_parametric_path("/foo/bar"), false);
        assert_eq!(is_parametric_path("/foo/bar/baz"), false);
    }

    #[test]
    fn test_is_parametric_path_on_parametric_paths() {
        assert_eq!(is_parametric_path("/foo/:id"), true);
        assert_eq!(is_parametric_path("/foo/:id/bar/baz"), true);
        assert_eq!(is_parametric_path("/foo/:foo_id/bar/:bar_id"), true);
    }

    #[test]
    fn test_static_path_matches_path() {
        let path = Path::new("/foo/bar");
        {
            let matches = path.matches("/foo");
            assert!(matches.is_none());
        }
        {
            let matches = path.matches("/foo/bar").unwrap();
            assert_eq!(matches.len(), 0);
        }
        {
            let matches = path.matches("/foo/bar/baz");
            assert!(matches.is_none());
        }
    }

    #[test]
    fn test_parametric_path_with_long_variable_matches_path() {
        let path = Path::new("/foo/:foooooooooooooooooooooooooooooid");
        {
            let matches = path.matches("/foo");
            assert!(matches.is_none());
        }
        {
            let params = path.matches("/foo/bar").unwrap();
            assert_eq!(
                params.get("foooooooooooooooooooooooooooooid").unwrap(),
                &"bar".to_string()
            );
        }
        {
            let params = path
                .matches("/foo/barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr")
                .unwrap();
            assert_eq!(
                params.get("foooooooooooooooooooooooooooooid").unwrap(),
                &"barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr".to_string()
            );
        }
        {
            let matches = path.matches("/foo/bar/baz");
            assert!(matches.is_none());
        }
    }

    #[test]
    fn test_parametric_path_with_short_variable_matches_path() {
        let path = Path::new("/foo/:a");
        {
            let matches = path.matches("/foo");
            assert!(matches.is_none());
        }
        {
            let params = path.matches("/foo/bar").unwrap();
            assert_eq!(params.get("a").unwrap(), &"bar".to_string());
        }
        {
            let params = path
                .matches("/foo/barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr")
                .unwrap();
            assert_eq!(
                params.get("a").unwrap(),
                &"barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr".to_string()
            );
        }
        {
            let matches = path.matches("/foo/bar/baz");
            assert!(matches.is_none());
        }
    }

    #[test]
    fn test_parametric_path_with_multiple_variables_matches_path() {
        let path = Path::new("/users/:user_id/friend/:friend_id/group/:group_id");
        let params = path.matches("/users/1/friend/2/group/77").unwrap();
        assert_eq!(params.get("user_id").unwrap(), &"1".to_string());
        assert_eq!(params.get("friend_id").unwrap(), &"2".to_string());
        assert_eq!(params.get("group_id").unwrap(), &"77".to_string());
    }

    #[test]
    fn test_parametric_path_with_only_variables() {
        let path = Path::new("/:one/:two/:three");
        {
            let matches = path.matches("/1/2/3").unwrap();
            assert_eq!(matches.get("one").unwrap(), &"1".to_string());
            assert_eq!(matches.get("two").unwrap(), &"2".to_string());
            assert_eq!(matches.get("three").unwrap(), &"3".to_string());
        }
        {
            let matches = path.matches("/hello/howdie/hey").unwrap();
            assert_eq!(matches.get("one").unwrap(), &"hello".to_string());
            assert_eq!(matches.get("two").unwrap(), &"howdie".to_string());
            assert_eq!(matches.get("three").unwrap(), &"hey".to_string());
        }
        {
            let matches = path.matches("/hello/hello/hello").unwrap();
            assert_eq!(matches.get("one").unwrap(), &"hello".to_string());
            assert_eq!(matches.get("two").unwrap(), &"hello".to_string());
            assert_eq!(matches.get("three").unwrap(), &"hello".to_string());
        }
        {
            let matches = path.matches("/hello");
            assert_eq!(matches.is_none(), true);
        }
        {
            let matches = path.matches("/hello/hello");
            assert_eq!(matches.is_none(), true);
        }
        {
            let matches = path.matches("/hello/hello/h/e/l/l/o");
            assert_eq!(matches.is_none(), true);
        }
    }

    #[test]
    fn test_parametric_path_with_variable_in_last_position_doesnt_glob_match() {
        let path = Path::new("/files/:path");
        let matches = path.matches("/files/home/user/file.text");
        assert_eq!(matches.is_none(), true);
    }

    #[test]
    fn test_parametric_path_with_variable_in_first_position_doesnt_glob_match() {
        let path = Path::new("/:path");
        let matches = path.matches("/home/user/file.text");
        assert_eq!(matches.is_none(), true);
    }
}
