use crate::csvtools::CsvParameters;
use csv::ReaderBuilder;
use std::collections::HashSet;


pub struct UniqueReader<'a>{
    data: HashSet<String>,
    file_parameters: CsvParameters<'a>,
    pub column: &'a str,
    column_index: usize,
    rows_in_file: i32
}


impl<'a> UniqueReader<'a>{
    
    pub fn new(file_parameters: CsvParameters<'a>, column: &'a str) -> UniqueReader<'a>{
        let column_index = file_parameters.column_index_from_header(column);
         UniqueReader{
            data: HashSet::new(),            
            file_parameters: file_parameters,
            column: column,
            column_index: column_index,
            rows_in_file: 0,                
        }
    }

    pub fn implement(&mut self){
        let mut rdr = ReaderBuilder::new().has_headers(self.file_parameters.has_headers).delimiter(self.file_parameters.separator as u8).from_path(self.file_parameters.file_path).unwrap();
        for result in rdr.records(){
            
            let record = result.unwrap();
            self.rows_in_file +=1;
            self.data.insert(record[self.column_index].to_string());                     
        }
    }

    pub fn print(&self){
        let mut records: Vec<&String> = self.data.iter().collect();
        records.sort();
        
        for record in records{
            println!("  {}", record);
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::Path;

    #[test]
fn test_unique(){
    let file_path = Path::new("test_data/example_short.csv");
    
    let params = CsvParameters::new(file_path, true, ';');
    let mut m = UniqueReader::new(params, "baz");
    m.implement();
    assert_eq!(HashSet::from(["1".to_string(), "2".to_string()]), m.data);


}
}     
