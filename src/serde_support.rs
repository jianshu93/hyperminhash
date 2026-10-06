use std::fmt;

use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{M, Sketch, zeroed_registers};

impl Serialize for Sketch {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // A tuple has a known length, avoiding a length prefix in formats such
        // as bincode. Borrow the registers instead of copying the array.
        let mut tuple = serializer.serialize_tuple(M as usize)?;
        for register in self.regs.iter() {
            tuple.serialize_element(register)?;
        }
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for Sketch {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SketchVisitor;

        impl<'de> Visitor<'de> for SketchVisitor {
            type Value = Sketch;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "exactly {M} u16 registers")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Sketch, A::Error> {
                // Do not deserialize into a stack array or allocate according
                // to an untrusted length hint from the input.
                let mut regs = zeroed_registers();
                for (index, register) in regs.iter_mut().enumerate() {
                    *register = seq
                        .next_element()?
                        .ok_or_else(|| A::Error::invalid_length(index, &self))?;
                }
                if seq.next_element::<u16>()?.is_some() {
                    return Err(A::Error::invalid_length(M as usize + 1, &self));
                }
                Ok(Sketch { regs })
            }
        }

        deserializer.deserialize_tuple(M as usize, SketchVisitor)
    }
}
