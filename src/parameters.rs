use std::collections::HashMap;

pub struct RouteParameters {
    params: HashMap<String, String>,
}

impl RouteParameters {
    pub(crate) fn none() -> RouteParameters {
        RouteParameters::with_capacity(0)
    }
    pub(crate) fn with_capacity(capacity: usize) -> RouteParameters {
        let map = HashMap::with_capacity(capacity);
        RouteParameters::new(map)
    }
    pub fn new(params: HashMap<String, String>) -> RouteParameters {
        RouteParameters { params }
    }
    pub fn len(&self) -> usize {
        self.params.len()
    }
    pub fn get(&self, param_name: &str) -> Option<&String> {
        self.params.get(param_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_route_parameters() {
        RouteParameters::none();

        let params = HashMap::new();
        RouteParameters::new(params);
    }

    #[test]
    fn test_length_of_parameters() {
        let mut map = HashMap::with_capacity(2);
        map.insert("foo".to_string(), "bar".to_string());
        map.insert("hello".to_string(), "world".to_string());
        let params = RouteParameters::new(map);
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_get_parameter() {
        let mut map = HashMap::with_capacity(2);
        map.insert("foo".to_string(), "bar".to_string());
        map.insert("hello".to_string(), "world".to_string());
        let params = RouteParameters::new(map);
        assert_eq!(params.get("foo").unwrap(), &"bar".to_string());
        assert_eq!(params.get("hello").unwrap(), &"world".to_string());
    }
}
