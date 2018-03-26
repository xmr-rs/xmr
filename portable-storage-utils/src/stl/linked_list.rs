// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::LinkedList;
use std::marker::PhantomData;
use std::fmt;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

use bytes::BytesMut;

use stl::StlElement;

#[derive(Debug, Default, Clone)]
pub struct StlLinkedList<T: StlElement>(pub LinkedList<T>);

impl<T> StlLinkedList<T>
    where T: StlElement
{
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> From<LinkedList<T>> for StlLinkedList<T>
    where T: StlElement
{
    fn from(ll: LinkedList<T>) -> StlLinkedList<T> {
        StlLinkedList(ll)
    }
}

impl<'de, T> Deserialize<'de> for StlLinkedList<T>
    where T: StlElement
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct LinkedListVisitor<T: StlElement>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for LinkedListVisitor<T>
            where T: StlElement
        {
            type Value = StlLinkedList<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a stl linked list")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: Error
            {
                let elements = v.len() / T::LENGTH;
                let mut list = LinkedList::new();
                for i in 0..elements {
                    let element_slice = &v[i * T::LENGTH..];
                    let element_slice = &element_slice[..T::LENGTH];

                    let element = T::from_bytes(element_slice).map_err(E::custom)?;
                    list.push_front(element);
                }

                Ok(StlLinkedList(list))
            }
        }

        deserializer.deserialize_bytes(LinkedListVisitor::<T>(PhantomData))
    }
}

impl<T> Serialize for StlLinkedList<T>
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
