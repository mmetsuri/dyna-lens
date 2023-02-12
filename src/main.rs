
use std::path::{PathBuf};
use clap::Parser;
use chrono::NaiveDateTime;

mod parser; 
mod stats; 
mod csvtools;
mod unique; 
mod filter; 
mod utils;
mod tasks;
mod comparison;

use csvtools::{CsvParameters};
use filter::{FilterReader};
use parser::filter_parser;
use stats::StatsReader;
use unique::UniqueReader;
use utils::print_columns;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    
    /// Csv file
    file_name: Option<PathBuf>,

    /// where clause
    #[clap(short='f', long="filter", value_name = "filter")]
    filter: Option<String>,
   
    #[clap(short='l', long)]
    list_columns: bool,

    #[clap(short='u', long, value_name = "unique")]
    unique: Option<String>,
    
    #[clap(long, value_name = "stats")]
    stats: Option<String>,

    // Default separator is comma
    #[clap(short='s', long, value_name = "separator")]
    separator: Option<char>,

    // Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,

}

fn main() {
    
    // TODO: Remove  input  folder
    // TODO: Add  some docs to README.md

    //if let Some((w, h)) = term_size::dimensions() {
    //    println!("Terminal width: {}, height: {}", w, h);
    //  } else {
    //    println!("Unable to get term size :(")
    //}

    let cli = Cli::parse();

    let file_name = match cli.file_name.as_deref(){
        Some(x) =>  x,
        None =>     {   println!("File name is required parameter. Exiting.");
                        return
                    }
    };
   
    let separator: char = match cli.separator{
        Some(x) => x,
        None => ','
    };

    let csv_parameters = CsvParameters::new(file_name, true, separator);

    if let Some(filter_clause) = cli.filter.as_deref(){

        let (operator, column_name, value) = filter_parser(filter_clause);
        if value.parse::<f64>().is_ok(){
            let mut reader: FilterReader<f64> = FilterReader::new(csv_parameters, column_name, operator, value.parse::<f64>().unwrap());
            reader.implement();
            reader.print();
        }
        else if value.parse::<NaiveDateTime>().is_ok(){
            let mut reader: FilterReader<NaiveDateTime> = FilterReader::new(csv_parameters, column_name, operator, value.parse::<NaiveDateTime>().unwrap());
            reader.implement();
            reader.print();
        }
        else{
            let mut reader: FilterReader<String> = FilterReader::new(csv_parameters, column_name, operator, value.parse::<String>().unwrap());
            reader.implement();
            reader.print();

        } 
    }

    else if let Some(unique_column) = cli.unique.as_deref(){
        let mut reader  = UniqueReader::new(csv_parameters, unique_column);
        reader.implement();
        reader.print();
    }

    else if let Some(stats_column) = cli.stats.as_deref(){
        let mut reader = StatsReader::new(csv_parameters, stats_column);
        reader.implement();
        reader.print();
    }
    else if cli.list_columns{
        print_columns(csv_parameters);
    }


}
