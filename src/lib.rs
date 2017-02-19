#[macro_use]
extern crate easy_csv_derive;
extern crate csv;

pub trait EasyCSV<T> {
    fn parse_csv<R: std::io::Read>(reader : &mut csv::Reader<R>) -> Vec<T>;
}

#[test]
fn test() {
    #[derive(Debug,EasyCSV,PartialEq)]
    struct Record {
        a: u32,
        b: i32
    }
    let mut rdr = csv::Reader::from_file("./test_input/test.csv").unwrap();
    let res = Record::parse_csv(&mut rdr);
    assert_eq!(res, vec![Record{a:2, b:-3}]);
}
