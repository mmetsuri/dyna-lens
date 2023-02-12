
use csv::{StringRecord, ReaderBuilder};
use std::path::Path;


pub struct CsvParameters<'a>{      
    pub file_path: &'a Path,
    pub has_headers: bool,
    pub separator: char,
    pub rows_in_file: i64,
}

impl<'a> CsvParameters<'a>{

    pub fn new(file_path: &'a Path, has_headers: bool, separator: char) -> CsvParameters{

        CsvParameters{
            file_path: file_path,
            has_headers: has_headers,
            separator: separator,
            rows_in_file: 0
        }
    }

    pub fn headers(&self) -> StringRecord {
        let separator = self.separator as u8;
        let mut rdr = ReaderBuilder::new().has_headers(self.has_headers).delimiter(separator).from_path(self.file_path).unwrap();
        let headers = match rdr.headers(){
            Ok(x) => x.clone(),
            Err(_) => panic!["Reading headers failed."]
        };
    
        headers
    }

    pub fn column_index_from_header(&self, column_name: &str) -> usize{

        for (idx, header) in self.headers().iter().enumerate(){
            if header == column_name{
                return idx
            }
        };
        panic!["Column {} not found.", column_name];
    }
    
}

#[test]
fn test_headers(){
    let file_path = Path::new("test_data/example_short.csv");
    
    let params = CsvParameters::new(file_path, true, ';');
    assert_eq!(params.headers(), vec!["foo", "bar",  "baz"]);
    assert_eq!(params.column_index_from_header("foo"), 0);
    assert_eq!(params.column_index_from_header("baz"), 2);

}

#[test]
#[should_panic]
fn test_header_index(){
    let file_path = Path::new("test_data/example_short.csv");
    
    let params = CsvParameters::new(file_path, true, ';');
    params.column_index_from_header("tuubaa");

}