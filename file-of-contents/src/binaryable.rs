pub trait Binaryable {
    fn to_bytes(&self) -> Vec<u8>;
}
