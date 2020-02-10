pub enum DataType<'a> {
    Driver(String),
    Trip(String, Vec<&'a str>),
    Unknown
}