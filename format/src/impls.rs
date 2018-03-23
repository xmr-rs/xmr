use de::{Deserialize, DeserializerStream, Error};
use ser::{Serialize, SerializerStream};

impl Deserialize for u64 {
    fn deserialize(deserializer: &mut DeserializerStream) -> Result<u64, Error> {
        deserializer.get_u64()
    }
}

impl Serialize for u64 {
    fn serialize(&self, mut serializer: SerializerStream) {
        serializer.put_u64(*self)
    }

    fn len(&self) -> usize {
        8
    }
}
