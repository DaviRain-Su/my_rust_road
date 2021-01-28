use serde;

// We will implement custom serialiation  and  deserialization
// for this struct
#[derive(Debug, PartialEq)]
struct KubeConfig {
    port: u8,
    healthz_port: u8,
    max_pods: u8,
}

///
///```
///pub trait Serialize {
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///         where S: Serializer;
///}
///```
///
///
pub fn hello_test() {
    println!("Hello, world");
}
/// Implementing Serialize for our custom struct defines
/// how instances of that struct should be serialized.
/// In essence, serialiation of an object is equal to
/// sum of the serialiations  of it's components
impl Serialize for KubeConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("KubeConfig", 3)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("healthz_port", &self.healthz_port)?;
        state.serialize_field("max_pods", &self.max_pods)?;
        state.end()
    }
}
fn main() {
    println!("Hello, world!");
}
