use csv::{StringRecord, ReaderBuilder};
use std::str::FromStr;
use std::fmt;

use crate::comparison::Comparison;
use crate::tasks::SubOperator;
use crate::csvtools::CsvParameters;


pub struct FilterReader<'a, T>{
    pub data: Vec<StringRecord>,
    //print_data: Vec<String>,
    file_parameters: CsvParameters<'a>,
    comparison_ord: T,
    operator: SubOperator,
    //column: &'a str,
    column_index: usize,
    pub rows_in_file: i64,
}

impl<'a,T> FilterReader<'a,T> where
    T: PartialOrd + Clone + FromStr, <T as FromStr>::Err: fmt::Debug{

        pub fn new(file_parameters: CsvParameters<'a>, column: &'a str, operator: SubOperator, comparison_ord: T) -> FilterReader<'a, T>{
     
            let column_index = file_parameters.column_index_from_header(column);
            
            FilterReader{
                data: Vec::new(),
                //print_data: Vec::new(),            
                file_parameters,
                comparison_ord, 
                //column,
                column_index,
                operator,
                rows_in_file: 0,                
            }
        }
             


    pub fn implement(&mut self){
        let mut rdr = ReaderBuilder::new().has_headers(self.file_parameters.has_headers).delimiter(self.file_parameters.separator as u8).from_path(self.file_parameters.file_path).unwrap();
        for result in rdr.records(){
                
            let record = result.unwrap();
            self.rows_in_file +=1;

            let test_value = &record[self.column_index];
            if !test_value.parse::<T>().is_ok(){
                continue
            };
            
            let value = test_value.parse::<T>().unwrap();
            if self.comparison_ord.comparison(&value, self.operator){
                self.data.push(record);
            }
       }
    }

    pub fn print(&self){

        let mut formatted_header: Vec<String> = Vec::new();   
        let mut formatted_values = Vec::new();
        let mut separator: Vec<String> = Vec::new();
        let mut max_column_width = Vec::new();
        let headers = self.file_parameters.headers();

        for (row_idx, row) in self.data.iter().enumerate(){
            for (col_idx, value) in row.iter().enumerate(){
                let len_value: i32 = i32::try_from(value.chars().count()).unwrap();
                if row_idx == 0{                    
                    let len_header: i32 = i32::try_from(headers[col_idx].chars().count()).unwrap();
                    if len_header > len_value{
                        max_column_width.push(len_header);
                    }else{
                        max_column_width.push(len_value);
                    }
                }else if len_value > max_column_width[col_idx] {
                    max_column_width[col_idx] = len_value;
                }
            }
        }
        
        for (idx, header) in headers.iter().enumerate(){
            let width=max_column_width[idx] as usize;
            formatted_header.push(format!("{:width$}", header.to_string(), width=width));
            separator.push(format!("{:-<width$}", "-", width=width));
        }

        for row in self.data.iter(){
            let mut formatted_row = Vec::new();
            for (col_idx, value) in row.iter().enumerate(){

                let len_value: i32 = i32::try_from(value.chars().count()).unwrap();
                let _len_diff = len_value - max_column_width[col_idx];
                //let mut formatted_value: String = value.to_string();

                let formatted_value = format!("{:width$}", value, width=max_column_width[col_idx] as usize);                
                formatted_row.push(formatted_value);

            }
            formatted_values.push(formatted_row.join("| "))

        }

        let header = formatted_header.join("| ");
        let separator = separator.join("|-");
        println!["{}",header];
        println!["{}",separator];
        for row in formatted_values{
            println!["{}",row];
        }
    }
    
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use chrono::NaiveDateTime;
    use super::*;

    #[test]
fn test_filtering(){
    let file_path = Path::new("test_data/example.csv");
    
    let params = CsvParameters::new(file_path, true, ',');
    let mut m = FilterReader::new(params, "Duration", SubOperator::GreaterThan, 600.0);
    m.implement();
    assert!(m.data.len() == 3);

}

#[test]
fn test_filtering_datetime(){
    let file_path = Path::new("test_data/example.csv");
    
    let params = CsvParameters::new(file_path, true, ',');
    let mut m: FilterReader<NaiveDateTime> = FilterReader::new(params, "Departure", SubOperator::LessThan, "2020-06-30T23:56:25".parse::<NaiveDateTime>().unwrap());
    m.implement();
    assert!(m.data.len() == 1);

}
}