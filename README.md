### Intro

Add a custom derive to a struct to make it parsable from CSVs. The derive
makes it possible to construct a CSVIterator over your type. The backend will
ignore extra columns as necessary, and only requires that the columns which
have fields in your struct exist. The String::parse<> functions are used to
parse CSV fields to the datatype of the field in your struct.

### Installation

```toml
[dependencies]
easy-csv = "0.3.2"
easy-csv-derive = "0.3.2"
csv = "0.15"
```

`csv` is a sibling dependency because you have to create the `csv::Reader` yourself.

### Usage example

```rust
extern crate easy_csv;
#[macro_use]
extern crate easy_csv_derive;
extern crate csv;

use easy_csv::{CSVIterator,CSVParsable};

#[derive(Debug,CSVParsable)]
struct Record {
    a : i32,
    b : i8,
    d : String,
}

fn main() {
    let data = "
a,b,c,d
2,-3,foo,bar";

    let mut rdr = csv::Reader::from_string(data);
    let iter = CSVIterator::<Record,_>::new(&mut rdr);
    let res : Vec<Record> = iter.collect();
    println!("{:?}", res);
}
```

Output: `[Record { a: 2, b: -3, d: "bar" }]`
