pub trait IdentifierProvidable {
    fn get_identifier(&self) -> [u8; 2];
}
