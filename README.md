easy-csv
===========

Add a custom derive to a struct to make it immediately parsable from CSVs. The
derive adds a method `parse_csv` which takes a `csv::Reader` and returns a
`Vec<T>` where `T` is your type. The backend will ignore extra columns as
necessary, and only requires that the columns which have fields in your struct
exist. The String::parse<> functions are used to parse CSV fields to the
datatype of the field in your struct.

Usage example:

```rust
#[macro_use]
extern crate easy_csv;
#[macro_use]
extern crate easy_csv_derive;
extern crate csv;

use easy_csv::EasyCSV;

#[derive(Debug,EasyCSV)]
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
    let res = Record::parse_csv(&mut rdr);
    println!("{:?}", res);
}
```

Will print `[Record { a: 2, b: -3, d: "bar" }]`
