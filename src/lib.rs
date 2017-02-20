#[macro_use]
#[allow(unused_imports)]
extern crate easy_csv_derive;
extern crate csv;

#[derive(Debug)]
pub enum Error {
    CSVError(csv::Error),
    MissingColumnError(String),
    ParseError(String)
}

impl std::convert::From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        Error::CSVError(e)
    }
}

pub trait CSVParsable<T> {
    fn parse_header<R: std::io::Read>(
        reader: &mut csv::Reader<R>) -> Result<Vec<usize>, ::Error>;
    fn parse_row<R: std::io::Read>(
        reader: &mut std::iter::Enumerate<csv::StringRecords<R>>,
        col_indices: &Vec<usize>) -> Option<Result<T, ::Error>>;
}

pub struct CSVIterator<'a, T, R> where T: CSVParsable<T>, R: 'a, R: std::io::Read {
    records: std::iter::Enumerate<csv::StringRecords<'a, R>>,
    col_indices: Vec<usize>,
    _marker: std::marker::PhantomData<T>
}

impl<'a, T, R> CSVIterator<'a, T, R> where T: CSVParsable<T>, R: 'a, R: std::io::Read {
    pub fn new(mut reader: &'a mut csv::Reader<R>) -> Result<Self, ::Error> {
        let col_indices = try!(T::parse_header(&mut reader));
        Ok(CSVIterator {
            records: reader.records::<'a>().into_iter().enumerate(),
            col_indices: col_indices,
            _marker: std::marker::PhantomData::<T> {}
        })
    }
}

impl<'a, T, R> Iterator for CSVIterator<'a, T, R> where T: CSVParsable<T>, R: std::io::Read {
    type Item=Result<T, ::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        T::parse_row(&mut self.records, &self.col_indices)
    }
}
