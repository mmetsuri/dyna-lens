

use crate::csvtools::CsvParameters;
use csv::{ReaderBuilder};
use statistical::{median, standard_deviation};
use rayon::prelude::*;


fn float_minmax(data: &Vec<f64>) -> Option<(f64, f64)>{
        
    let mut floats: Vec<&f64> = data.iter().filter(|x| !x.is_nan()).collect();
    
    // If all values are nan, we have an empty vec
    if floats.len() == 0{
        return None
    }
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Some( (floats[0].clone(), floats[floats.len()-1].clone()))
}


pub struct StatsReader<'a>{
    data: Vec<f64>,
    file_parameters: CsvParameters<'a>,
    pub mean: f64,
    pub std: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub column: &'a str,
    column_index: usize,
    rows_in_file: i32
}




impl<'a> StatsReader<'a> where
    
    {
    
    pub fn new(file_parameters: CsvParameters<'a>, column: &'a str) -> StatsReader<'a>{
        let column_index = file_parameters.column_index_from_header(column);
         StatsReader{
            data: Vec::new(),            
            file_parameters: file_parameters,
            mean: 0.0,
            std: 0.0,
            median: 0.0,
            min: 0.0,
            max: 0.0,
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


                let test_value = &record[self.column_index];
                if !test_value.parse::<f64>().is_ok(){
                    // Test if string is float. Skip rows which are not.
                    continue
                }
                let value = test_value.parse::<f64>().unwrap();
                self.data.push(value);
        }        
        
        self.mean = self.data.par_iter().sum::<f64>()/self.data.len() as f64; 
        self.median = median(&self.data); 
        self.std = standard_deviation(&self.data, None);
        let (min, max) = float_minmax(&self.data).unwrap();
        self.min = min;
        self.max = max;

    }

    pub fn print(&self){
        println!["Mean {}", self.mean];
        println!["Median {}", self.median];
        println!["Standard deviation {}", self.std];
        println!["Minimum value {}", self.min];
        println!["Maximum value {}", self.max];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_statistics(){
        let file_path = Path::new("test_data/example_short.csv");
        
        let params = CsvParameters::new(file_path, true, ';');
        let mut m = StatsReader::new(params, "foo");
        m.implement();

        assert_eq!(3.5, m.mean);
        assert_eq!(3.5, m.median);
        assert_eq!(1.8708286933869707, m.std);
        assert_eq!(6.0, m.max);
        assert_eq!(1.0, m.min);

    }

    #[test]
    fn test_minmax(){
        let f1 = vec![0.0, 1.0, 2.0];
        let (min, max) = float_minmax(&f1).unwrap();
        assert_eq!(min, 0.0);
        assert_eq!(max, 2.0);

        let f1 = vec![0.0, 1.0, 2.0, f64::NAN];
        let (min, max) = float_minmax(&f1).unwrap();
        assert_eq!(min, 0.0);
        assert_eq!(max, 2.0);

    }
}