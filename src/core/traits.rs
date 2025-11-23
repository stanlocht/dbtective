pub trait Descriptable {
    fn description(&self) -> Option<&String>;
    fn get_object_string(&self) -> String;
}
