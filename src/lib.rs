#[macro_use]
extern crate easy_csv_derive;
extern crate csv;

pub trait CSVParsable<T> {
    fn parse_header<R: std::io::Read>(reader: &mut csv::Reader<R>) -> Vec<usize>;
    fn parse_row<R: std::io::Read>(
        reader: &mut csv::StringRecords<R>,
        col_indices: &Vec<usize>) -> Option<T>;
}

pub struct CSVIterator<'a, T, R> where T: CSVParsable<T>, R: 'a, R: std::io::Read {
    records: csv::StringRecords<'a, R>,
    col_indices: Vec<usize>,
    _marker: std::marker::PhantomData<T>
}

impl<'a, T, R> CSVIterator<'a, T, R> where T: CSVParsable<T>, R: 'a, R: std::io::Read {
    pub fn new(mut reader: &'a mut csv::Reader<R>) -> Self {
        let col_indices = T::parse_header(&mut reader);
        CSVIterator {
            records: reader.records::<'a>(),
            col_indices: col_indices,
            _marker: std::marker::PhantomData::<T> {}
        }
    }
}

impl<'a, T, R> Iterator for CSVIterator<'a, T, R> where T: CSVParsable<T>, R: std::io::Read {
    type Item=T;

    fn next(&mut self) -> Option<Self::Item> {
        T::parse_row(&mut self.records, &self.col_indices)
    }
}

#[test]
fn test() {
    #[derive(Debug,CSVParsable,PartialEq)]
    struct Record {
        a: u32,
        b: i32
    }
    let mut rdr = csv::Reader::from_file("./test_input/test.csv").unwrap();
    let iter = CSVIterator::<Record,_>::new(&mut rdr);
    let res : Vec<Record> = iter.collect();
    assert_eq!(res, vec![Record { a: 2, b: -3 }]);
}
