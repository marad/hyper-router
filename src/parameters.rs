pub struct RouteParameters {
    pub parameters: Vec<String>,
}

impl RouteParameters {
    pub fn new(params: Vec<String>) -> RouteParameters {
        RouteParameters { parameters: params }
    }
}
