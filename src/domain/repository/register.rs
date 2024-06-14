pub trait RegisterRepository {
    fn register(&self);
    fn get_card(&self) -> String;
    fn is_register_mode(&self) -> bool;
    fn add_card(&self, card: String);
}
