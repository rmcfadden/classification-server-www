pub trait Serialize<T> {
    fn serialize(&self) -> T;
}