//! Build the pretty tree from a `serde::Serialize` value (feature `serde`).
//!
//! A small `Serializer` turns each serde call into the same tree the
//! `Debug` parser produces, with every leaf formatted the way `Debug`
//! formats it, so derived types print the same either way.

use std::fmt::{self, Display};

use serde::ser::{self, Serialize};

use super::{Container, Item, Kind, Value, MAX_PARSE_DEPTH};

/// Convert a value; a serialization error becomes an atom describing it.
pub(super) fn to_value<T: Serialize + ?Sized>(value: &T) -> Value {
    value
        .serialize(ValueSerializer { depth: 0 })
        .unwrap_or_else(|e| Value::Atom(format!("<serialize error: {e}>")))
}

#[derive(Debug)]
pub(super) struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

fn atom(s: impl ToString) -> Value {
    Value::Atom(s.to_string())
}

fn container(kind: Kind, items: Vec<Item>) -> Value {
    Value::Container(Box::new(Container { kind, items }))
}

#[derive(Clone, Copy)]
struct ValueSerializer {
    depth: usize,
}

impl ValueSerializer {
    fn child<T: Serialize + ?Sized>(self, value: &T) -> Result<Value, Error> {
        value.serialize(ValueSerializer {
            depth: self.depth + 1,
        })
    }

    fn compound(self, kind: Kind) -> Compound {
        Compound {
            ser: self,
            kind,
            items: Vec::new(),
            key: None,
        }
    }
}

/// Collects the children of a seq, tuple, map or struct.
struct Compound {
    ser: ValueSerializer,
    kind: Kind,
    items: Vec<Item>,
    key: Option<Value>,
}

impl Compound {
    fn too_deep(&self) -> bool {
        self.ser.depth >= MAX_PARSE_DEPTH
    }

    fn push<T: Serialize + ?Sized>(&mut self, key: Option<Value>, value: &T) -> Result<(), Error> {
        if !self.too_deep() {
            let value = self.ser.child(value)?;
            self.items.push(Item { key, value });
        }
        Ok(())
    }

    fn finish(self) -> Result<Value, Error> {
        if self.too_deep() {
            return Ok(atom("..."));
        }
        // Derived Debug prints a struct or variant without fields as its
        // bare name (`Unit`, `Empty`).
        Ok(match self.kind {
            Kind::Struct(name) | Kind::TupleStruct(name) if self.items.is_empty() => atom(name),
            kind => container(kind, self.items),
        })
    }
}

macro_rules! display_atoms {
    ($($method:ident: $ty:ty),* $(,)?) => {
        $(fn $method(self, v: $ty) -> Result<Value, Error> {
            Ok(atom(v))
        })*
    };
}

impl ser::Serializer for ValueSerializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = Compound;
    type SerializeTuple = Compound;
    type SerializeTupleStruct = Compound;
    type SerializeTupleVariant = Compound;
    type SerializeMap = Compound;
    type SerializeStruct = Compound;
    type SerializeStructVariant = Compound;

    display_atoms! {
        serialize_bool: bool,
        serialize_i8: i8, serialize_i16: i16, serialize_i32: i32,
        serialize_i64: i64, serialize_i128: i128,
        serialize_u8: u8, serialize_u16: u16, serialize_u32: u32,
        serialize_u64: u64, serialize_u128: u128,
    }

    fn serialize_f32(self, v: f32) -> Result<Value, Error> {
        Ok(Value::Atom(format!("{v:?}")))
    }

    fn serialize_f64(self, v: f64) -> Result<Value, Error> {
        Ok(Value::Atom(format!("{v:?}")))
    }

    fn serialize_char(self, v: char) -> Result<Value, Error> {
        Ok(Value::Atom(format!("{v:?}")))
    }

    fn serialize_str(self, v: &str) -> Result<Value, Error> {
        Ok(Value::Str(format!("{v:?}")))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Value, Error> {
        // Debug shows bytes as a list of numbers.
        Ok(container(
            Kind::List,
            v.iter()
                .map(|b| Item {
                    key: None,
                    value: atom(b),
                })
                .collect(),
        ))
    }

    fn serialize_none(self) -> Result<Value, Error> {
        Ok(atom("None"))
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Value, Error> {
        self.serialize_newtype_struct("Some", value)
    }

    fn serialize_unit(self) -> Result<Value, Error> {
        Ok(container(Kind::Tuple, Vec::new()))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Value, Error> {
        Ok(atom(name))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Value, Error> {
        Ok(atom(variant))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Value, Error> {
        let mut compound = self.compound(Kind::TupleStruct(name.into()));
        compound.push(None, value)?;
        compound.finish()
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value, Error> {
        self.serialize_newtype_struct(variant, value)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Compound, Error> {
        Ok(self.compound(Kind::List))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Compound, Error> {
        Ok(self.compound(Kind::Tuple))
    }

    fn serialize_tuple_struct(self, name: &'static str, _len: usize) -> Result<Compound, Error> {
        Ok(self.compound(Kind::TupleStruct(name.into())))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Compound, Error> {
        Ok(self.compound(Kind::TupleStruct(variant.into())))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Compound, Error> {
        Ok(self.compound(Kind::Brace))
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Compound, Error> {
        Ok(self.compound(Kind::Struct(name.into())))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Compound, Error> {
        Ok(self.compound(Kind::Struct(variant.into())))
    }
}

impl ser::SerializeSeq for Compound {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(None, value)
    }

    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}

impl ser::SerializeTuple for Compound {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(None, value)
    }

    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}

impl ser::SerializeTupleStruct for Compound {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(None, value)
    }

    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}

impl ser::SerializeTupleVariant for Compound {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        self.push(None, value)
    }

    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}

impl ser::SerializeMap for Compound {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
        if !self.too_deep() {
            self.key = Some(self.ser.child(key)?);
        }
        Ok(())
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        let key = self.key.take();
        self.push(key, value)
    }

    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}

impl ser::SerializeStruct for Compound {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.push(Some(atom(key)), value)
    }

    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}

impl ser::SerializeStructVariant for Compound {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.push(Some(atom(key)), value)
    }

    fn end(self) -> Result<Value, Error> {
        self.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::super::{layout, parse, PrettyOptions};
    use super::*;
    use serde::Serialize;
    use std::collections::BTreeMap;

    #[derive(Debug, Serialize)]
    struct Point {
        x: i32,
        y: f64,
    }

    #[derive(Debug, Serialize)]
    struct Unit;

    #[derive(Debug, Serialize)]
    struct Meters(u32);

    #[derive(Debug, Serialize)]
    enum Shape {
        Empty,
        Circle(f32),
        Rect { w: u8, h: u8 },
        Pair(i8, i8),
    }

    /// The serde tree and the parsed Debug tree lay out identically.
    fn same_as_debug<T: Serialize + std::fmt::Debug>(value: &T) {
        for w in [0, 8, 20, 80, usize::MAX] {
            let options = PrettyOptions::default().max_width(w);
            let debug = layout(&parse(&format!("{value:?}")), &options);
            let serde = layout(&to_value(value), &options);
            assert_eq!(serde, debug, "width {w}");
        }
    }

    #[test]
    fn derived_types_match_debug() {
        same_as_debug(&Point { x: -1, y: 2.5 });
        same_as_debug(&Unit);
        same_as_debug(&Meters(5));
        same_as_debug(&vec![
            Shape::Empty,
            Shape::Circle(1.0),
            Shape::Rect { w: 2, h: 3 },
            Shape::Pair(-1, 1),
        ]);
        same_as_debug(&(1,));
        same_as_debug(&());
        same_as_debug(&(Some("a\"b\n"), None::<u8>, 'x', 'ß'));
        let mut m = BTreeMap::new();
        m.insert((1, "k"), vec![Some(1u128), None]);
        same_as_debug(&m);
        same_as_debug(&Vec::<u8>::new());
    }

    #[test]
    fn deep_values_are_capped() {
        let v = nest(MAX_PARSE_DEPTH + 10);
        let out = layout(&to_value(&v), &PrettyOptions::default());
        assert!(out.contains("[...]") || out.contains("..."), "{out}");
    }

    /// Nested single-element vectors, built without recursion.
    fn nest(depth: usize) -> Vec<Nest> {
        let mut v = Vec::new();
        for _ in 0..depth {
            v = vec![Nest(v)];
        }
        v
    }

    #[derive(Serialize)]
    struct Nest(Vec<Nest>);

    #[test]
    fn errors_become_atoms() {
        struct Bad;
        impl Serialize for Bad {
            fn serialize<S: ser::Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
                Err(ser::Error::custom("nope"))
            }
        }
        assert_eq!(
            layout(&to_value(&Bad), &PrettyOptions::default()),
            "<serialize error: nope>"
        );
    }
}
