/// Unminified Drizzle CSS.
pub const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/drizzle.css"));

/// Minified Drizzle CSS.
pub const CSS_MIN: &str = include_str!(concat!(env!("OUT_DIR"), "/drizzle.min.css"));
