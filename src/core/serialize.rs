pub trait Serialize<T> {
    fn serialize(&self) -> T;
    fn deserialize(&mut self, input: T );
}
