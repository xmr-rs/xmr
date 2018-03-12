use std::marker::PhantomData;
use std::fmt;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

use bytes::BytesMut;

use stl::StlElement;

#[derive(Debug, Default, Clone)]
pub struct StlVector<T: StlElement>(pub Vec<T>);

impl<'de, T> Deserialize<'de> for StlVector<T>
    where T: StlElement
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct VectorVisitor<T: StlElement>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for VectorVisitor<T>
            where T: StlElement
        {
            type Value = StlVector<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a stl linked list")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: Error
            {
                let elements = v.len() / T::LENGTH;
                let mut vector = Vec::new();
                for i in 0..elements {
                    let element_slice = &v[i * T::LENGTH..];
                    let element_slice = &element_slice[..T::LENGTH];

                    let element = T::from_bytes(element_slice).map_err(E::custom)?;
                    vector.push(element);
                }

                Ok(StlVector(vector))
            }
        }

        deserializer.deserialize_bytes(VectorVisitor::<T>(PhantomData))
    }
}

impl<T> Serialize for StlVector<T>
    where T: StlElement
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut buf = BytesMut::with_capacity(T::LENGTH * self.0.len());
        for element in self.0.iter() {
            element.to_bytes(&mut buf);
        }

        serializer.serialize_bytes(buf.as_ref())
    }
}
