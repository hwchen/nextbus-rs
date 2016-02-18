
// Components of Route

/// A stop along a Route
#[derive(Debug)]
pub struct Stop {
    tag: String,
    title: String,
    lat: String,
    long: String,
    stop_id: String,
}

/// An itinerary along a route
#[derive(Debug)]
pub struct Direction {
    tag: String,
    title: String,
    name: String,
    use_for_ui: bool,
    // stops is an ordered vector of stop tags
    // References are kept because stops will
    // be referenced many times among the many
    // directions
    stops: Vec<String>,
}

/// The coordinates tracing a Route
#[derive(Debug)]
pub struct Path(Vec<Point>);

/// A coordinate in a Path
#[derive(Debug)]
pub struct Point {
    lat: f32,
    lon: f32,
}
