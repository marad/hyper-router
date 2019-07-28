use crate::parameters::RouteParameters;

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
                    Some(RouteParameters::new(vec![]))
                } else {
                    None
                }
            }
            Path::Parametric(self_path) => {
                let mut params = vec![];
                let mut self_segments = self_path.iter();
                let mut other_segments = other_path.split('/');

                loop {
                    let self_seg = self_segments.next();
                    let other_seg = other_segments.next();

                    match (self_seg, other_seg) {
                        // We have two segments to compare.
                        (Some(left), Some(right)) => {
                            let first_self_char = left.chars().nth(0);
                            match first_self_char {
                                Some(ch) if ch == ':' => {
                                    params.push(right.to_string());
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
            let matches = path.matches("/foo/bar");
            let empty_vec: Vec<String> = vec![];
            assert!(matches.is_some());
            assert_eq!(matches.unwrap().parameters, empty_vec);
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
            let matches = path.matches("/foo/bar");
            assert!(matches.is_some());
            assert_eq!(matches.unwrap().parameters, vec!["bar"]);
        }
        {
            let matches = path.matches("/foo/barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr");
            assert!(matches.is_some());
            assert_eq!(
                matches.unwrap().parameters,
                vec!["barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr"]
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
            let matches = path.matches("/foo/bar");
            assert!(matches.is_some());
            assert_eq!(matches.unwrap().parameters, vec!["bar"]);
        }
        {
            let matches = path.matches("/foo/barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr");
            assert!(matches.is_some());
            assert_eq!(
                matches.unwrap().parameters,
                vec!["barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr"]
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
        let matches = path.matches("/users/1/friend/2/group/77").unwrap();
        assert_eq!(matches.parameters, vec!["1", "2", "77"]);
    }

    #[test]
    fn test_parametric_path_with_only_variables() {
        let path = Path::new("/:one/:two/:three");
        {
            let matches = path.matches("/1/2/3").unwrap();
            assert_eq!(matches.parameters, vec!["1", "2", "3"]);
        }
        {
            let matches = path.matches("/hello/howdie/hey").unwrap();
            assert_eq!(matches.parameters, vec!["hello", "howdie", "hey"]);
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
    fn test_parametric_path_with_variable_in_last_position_matches_multiple_segments_as_one() {
        let path = Path::new("/files/:path");
        let matches = path.matches("/files/home/user/file.text").unwrap();
        assert_eq!(matches.parameters, vec!["home/user/file.text"]);
    }
}
