use crate::csvtools::CsvParameters;


pub fn print_columns<'a>(csv: CsvParameters){

    let headers =  csv.headers();
    println!["==========================="];
    println!["Columns"];
    println!["---------------------------"];
    for column in headers.iter(){
        println!("{}", column);
    }
    println!["==========================="];

}