pub trait Descriptable {
    fn description(&self) -> Option<&String>;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
}
