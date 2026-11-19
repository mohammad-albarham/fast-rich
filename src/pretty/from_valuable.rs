//! Build the pretty tree from a `valuable::Valuable` value (feature
//! `valuable`).
//!
//! `valuable` exposes a value's structure (struct and field names, enum
//! variants, lists, maps, tuples) through its `Visit` trait. Each part is
//! turned into the same tree the `Debug` parser produces, with leaves
//! formatted the way `Debug` formats them.

use valuable::{Fields, NamedValues, Valuable, Visit};

use super::{Container, Item, Kind, Value, MAX_PARSE_DEPTH};

/// Convert a value.
pub(super) fn to_value<T: Valuable + ?Sized>(value: &T) -> Value {
    convert(value.as_value(), 0)
}

fn container(kind: Kind, items: Vec<Item>) -> Value {
    Value::Container(Box::new(Container { kind, items }))
}

/// Collects the children reported by a `visit` call.
struct Collector {
    depth: usize,
    items: Vec<Item>,
}

impl Collector {
    fn collect(depth: usize, value: &dyn Valuable) -> Vec<Item> {
        let mut collector = Collector {
            depth,
            items: Vec::new(),
        };
        value.visit(&mut collector);
        collector.items
    }

    fn push(&mut self, key: Option<Value>, value: valuable::Value<'_>) {
        let value = convert(value, self.depth + 1);
        self.items.push(Item { key, value });
    }
}

impl Visit for Collector {
    fn visit_value(&mut self, value: valuable::Value<'_>) {
        self.push(None, value);
    }

    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        for (field, value) in named_values {
            self.push(Some(Value::Atom(field.name().to_string())), *value);
        }
    }

    fn visit_unnamed_fields(&mut self, values: &[valuable::Value<'_>]) {
        for value in values {
            self.push(None, *value);
        }
    }

    fn visit_entry(&mut self, key: valuable::Value<'_>, value: valuable::Value<'_>) {
        let key = convert(key, self.depth + 1);
        self.push(Some(key), value);
    }
}

/// A struct or enum variant: `Name { .. }`, `Name(..)`, or a bare `Name`
/// when it has no fields (as derived `Debug` prints it).
fn named(name: &str, fields: &Fields<'_>, depth: usize, value: &dyn Valuable) -> Value {
    let kind = match fields {
        Fields::Named(_) => Kind::Struct(name.to_string()),
        Fields::Unnamed(_) => Kind::TupleStruct(name.to_string()),
    };
    let items = Collector::collect(depth, value);
    if items.is_empty() {
        Value::Atom(name.to_string())
    } else {
        container(kind, items)
    }
}

fn convert(value: valuable::Value<'_>, depth: usize) -> Value {
    use valuable::Value as V;

    let is_container = matches!(
        value,
        V::Listable(_) | V::Mappable(_) | V::Structable(_) | V::Enumerable(_) | V::Tuplable(_)
    );
    if is_container && depth >= MAX_PARSE_DEPTH {
        return Value::Atom("...".into());
    }

    match value {
        V::Bool(v) => Value::Atom(v.to_string()),
        V::Char(v) => Value::Atom(format!("{v:?}")),
        V::F32(v) => Value::Atom(format!("{v:?}")),
        V::F64(v) => Value::Atom(format!("{v:?}")),
        V::I8(v) => Value::Atom(v.to_string()),
        V::I16(v) => Value::Atom(v.to_string()),
        V::I32(v) => Value::Atom(v.to_string()),
        V::I64(v) => Value::Atom(v.to_string()),
        V::I128(v) => Value::Atom(v.to_string()),
        V::Isize(v) => Value::Atom(v.to_string()),
        V::U8(v) => Value::Atom(v.to_string()),
        V::U16(v) => Value::Atom(v.to_string()),
        V::U32(v) => Value::Atom(v.to_string()),
        V::U64(v) => Value::Atom(v.to_string()),
        V::U128(v) => Value::Atom(v.to_string()),
        V::Usize(v) => Value::Atom(v.to_string()),
        V::String(v) => Value::Str(format!("{v:?}")),
        V::Path(v) => Value::Str(format!("{v:?}")),
        // An error has no structure in valuable; parse its Debug output.
        V::Error(e) => super::parse(&format!("{e:?}")),
        // valuable maps `None` to `Unit` (and `Some(x)` to `x`).
        V::Unit => Value::Atom("None".into()),
        V::Listable(list) => container(Kind::List, Collector::collect(depth, list)),
        V::Mappable(map) => container(Kind::Brace, Collector::collect(depth, map)),
        V::Tuplable(tuple) => container(Kind::Tuple, Collector::collect(depth, tuple)),
        V::Structable(s) => {
            let def = s.definition();
            named(def.name(), def.fields(), depth, s)
        }
        V::Enumerable(e) => {
            let variant = e.variant();
            named(variant.name(), variant.fields(), depth, e)
        }
        // `valuable::Value` is non-exhaustive.
        other => Value::Atom(format!("{other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{layout, parse, PrettyOptions};
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Debug, Valuable)]
    struct Point {
        x: i32,
        y: f64,
    }

    #[derive(Debug, Valuable)]
    struct Unit;

    #[derive(Debug, Valuable)]
    struct Meters(u32);

    #[derive(Debug, Valuable)]
    enum Shape {
        Empty,
        Circle(f32),
        Rect { w: u8, h: u8 },
        Pair(i8, i8),
    }

    fn same_as_debug<T: Valuable + std::fmt::Debug>(value: &T) {
        for w in [0, 8, 20, 80, usize::MAX] {
            let options = PrettyOptions::default().max_width(w);
            let debug = layout(&parse(&format!("{value:?}")), &options);
            let valuable = layout(&to_value(value), &options);
            assert_eq!(valuable, debug, "width {w}");
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
        same_as_debug(&("a\"b\n", 'x', 'ß'));
        let mut m = BTreeMap::new();
        m.insert("k", vec![1u128, 2]);
        same_as_debug(&m);
        same_as_debug(&Vec::<u8>::new());
    }

    #[test]
    fn options_are_flattened() {
        // valuable has no Some/None: `Some(x)` is `x`, `None` is `Unit`.
        let options = PrettyOptions::default();
        assert_eq!(
            layout(&to_value(&vec![Some(1), None]), &options),
            "[1, None]"
        );
    }

    #[test]
    fn deep_values_are_capped() {
        #[derive(Valuable)]
        struct Nest(Vec<Nest>);
        let mut v = Vec::new();
        for _ in 0..MAX_PARSE_DEPTH + 10 {
            v = vec![Nest(v)];
        }
        let out = layout(&to_value(&v), &PrettyOptions::default());
        assert!(out.contains("..."), "{out}");
    }
}
