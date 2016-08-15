//! An optional implementation of serialization/deserialization. Reference
//! implementations used:
//!
//! - [Serialize][1].
//! - [Deserialize][2].
//!
//! [1]: https://github.com/serde-rs/serde/blob/97856462467db2e90cf368e407c7ebcc726a01a9/serde/src/ser/impls.rs#L601-L611
//! [2]: https://github.com/serde-rs/serde/blob/97856462467db2e90cf368e407c7ebcc726a01a9/serde/src/de/impls.rs#L694-L746

extern crate serde;

use super::LinearMap;
use super::set::LinearSet;

use self::serde::{Serialize, Serializer, Deserialize, Deserializer};
use self::serde::de::{Visitor, MapVisitor, SeqVisitor, Error};

use std::marker::PhantomData;

impl<K, V> Serialize for LinearMap<K, V>
    where K: Serialize + Eq,
          V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        let mut state = try!(serializer.serialize_map(Some(self.len())));
        for (k, v) in self {
            try!(serializer.serialize_map_key(&mut state, k));
            try!(serializer.serialize_map_value(&mut state, v));
        }
        serializer.serialize_map_end(state)
    }
}

#[allow(missing_docs)]
pub struct LinearMapVisitor<K, V> {
    marker: PhantomData<LinearMap<K, V>>,
}

impl<K, V> LinearMapVisitor<K, V> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        LinearMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<K, V> Visitor for LinearMapVisitor<K, V>
    where K: Deserialize + Eq,
          V: Deserialize,
{
    type Value = LinearMap<K, V>;

    #[inline]
    fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
        where E: Error,
    {
        Ok(LinearMap::new())
    }

    #[inline]
    fn visit_map<Visitor>(&mut self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
        where Visitor: MapVisitor,
    {
        let mut values = LinearMap::with_capacity(visitor.size_hint().0);

        while let Some((key, value)) = try!(visitor.visit()) {
            values.insert(key, value);
        }

        try!(visitor.end());

        Ok(values)
    }
}

impl<K, V> Deserialize for LinearMap<K, V>
    where K: Deserialize + Eq,
          V: Deserialize,
{
    fn deserialize<D>(deserializer: &mut D) -> Result<LinearMap<K, V>, D::Error>
        where D: Deserializer,
    {
        deserializer.deserialize_map(LinearMapVisitor::new())
    }
}


impl<K> Serialize for LinearSet<K>
    where K: Serialize + Eq
{
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        let mut state = try!(serializer.serialize_seq(Some(self.len())));
        for e in self {
            try!(serializer.serialize_seq_elt(&mut state, e));
        }
        serializer.serialize_seq_end(state)
    }
}


#[allow(missing_docs)]
pub struct LinearSetVisitor<K> {
    marker: PhantomData<LinearSet<K>>,
}

impl<K> LinearSetVisitor<K>{
    #[allow(missing_docs)]
    pub fn new() -> Self {
        LinearSetVisitor {
            marker: PhantomData,
        }
    }
}

impl<K> Visitor for LinearSetVisitor<K>
    where K: Deserialize + Eq,
{
    type Value = LinearSet<K>;

    #[inline]
    fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
        where E: Error,
    {
        Ok(LinearSet::new())
    }

    #[inline]
    fn visit_seq<Visitor>(&mut self, mut visitor: Visitor) -> Result<Self::Value, Visitor::Error>
        where Visitor: SeqVisitor,
    {
        let mut values = LinearSet::with_capacity(visitor.size_hint().0);

        while let Some(key) = try!(visitor.visit()) {
            values.insert(key);
        }

        try!(visitor.end());

        Ok(values)
    }
}

impl<K> Deserialize for LinearSet<K>
    where K: Deserialize + Eq,
{
    fn deserialize<D>(deserializer: &mut D) -> Result<LinearSet<K>, D::Error>
        where D: Deserializer,
    {
        deserializer.deserialize_seq(LinearSetVisitor::new())
    }
}
