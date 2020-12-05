# binserde

A crate similar to serde, but specialized for serializing into a compact
binary format, including features like string deduplication.

*This crate is very WIP.* Features currently not implemented but planned
include incremental versioning support so that old formats can still be
loaded when the data format changes, deduplication of arbitrary data
structures, and explicit tagging (writing a struct or enum as a set of
key/value pairs instead of serializing the items in order of declaration,
for higher resistance to format changes at the expense of output size)

## Usage

```
use std::fs::File;
use std::io::BufReader;

#[derive(BinSerialize, BinDeserialize, Eq, PartialEq)]
struct MyData {
    v1: String,
    v2: Option<usize>,
}

let my_data = MyData {
    v1: "Some Text".to_string(),
    v2: Some(12415165),
};

let vec = binserde::serialize(&my_data).unwrap();

let copy_of_my_data: MyData = binserde::deserialize(&vec).unwrap();

assert_eq!(my_data, copy_of_my_data);
```

## Macro Attributes

`#[derive(BinSerialize)]` and `#[derive(BinDeserialize)]` allows using
attributes on the type itself and its fields to control (de)serialization.

### `#[binserde(skip)]`

Valid for: fields

Skips the field when serializing. When deserializing, uses
[`Default::default()`] instead of reading from the stream to fill the field.

### `#[binserde(no_dedup)]`

Valid for: fields

Turns off deduplication for this field. See [Deduplication] for more
information about how it works.

### `#[binserde(index = n)]`

Valid for: fields

**not implemented**

Moves the field and all following fields to the specified position `n` when
serializing, shifting everything originally after that position to the
right.

#### Example:

```
#[derive(BinSerialize)]
struct S {
    w: u8,
    x: u8,
    #[binserde(index = 0)]
    y: u8,
    z: u8,
}

let vec = binserde::serialize(&S { w: 0, x: 1, y: 2, z: 3 });

assert_eq!(&[2, 3, 0, 1], &vec);
```

The attribute moved `y` and `z` into position 0, pushing `w` and `x` back to
positions 2 and 3 respectively.

The attribute can be applied on more than one field, in which case moving
operations will be evaluated from top to bottom. That means, the following
struct serializes in the order z, x, y, w and not x, y, z, w or any other
order:

```
#[derive(BinSerialize)]
struct S {
    w: u8,
    #[binserde(index = 0)]
    x: u8,
    y: u8,
    #[binserde(index = 0)]
    z: u8,
}
```

# Deduplication

Deduplication is currently only implemented for strings. It works by taking
any [`String`] or [`str`] that is serialized using its [`BinSerializer`]
implementation and adds it to a seperate list which is written to the
beginning of the buffer given to [`serialize`] (or an equivalent function,
after which the actual data follows. In that data, the string is replaced by
a `usize` pointing to the index in the string list. Effectively, a
deduplicated data structure gets transformed from this:

```
struct S {
    s1: String,
    s2: String,
    strs: Vec<String>,
    something_else: u32,
}
```

to this:

```
struct S1 {
    strings: Vec<String>,
    s1: usize,
    s2: usize,
    strs: Vec<usize>,
    something_else: u32,
}
```

when serializing. This can have a major impact on the resulting size of the
serialized data structure when multiple occurrences of the same string
appear.

