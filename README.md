# DynaLens
DynaLens is a command line tool for reading and basic analysis of CSV (comma separated) files. It can, for example, filter data based on certain column, calculate column averages or list unique values in a column.

The aim at the moment is to test command line user interface for queries. DynLens is a prototype, with many basic features missing.

## Examples
Below is an example data set, stored in _test_data/bike_data.csv_.
```
Departure,Departure station id,Departure station name,Duration(s)
2020-06-30T23:59:20,34,Kansallismuseo,887
2020-06-30T23:58:21,519,Tapionaukio,478
2020-06-30T23:55:31,5,Sepänkatu,631
2020-06-30T22:09:39,201,Länsisatamankuja,566
2020-06-30T22:09:37,565,Mankkaanaukio,838
```

Let's filter the data
```
$ ./dynalens bike_data.csv --filter "Duration(s)<500"
```
Lens will output rows that fulfill the condition for Duration column
```
Departure          | Departure station id| Departure station name| Duration(s)
-------------------|---------------------|-----------------------|------------
2020-06-30T23:58:21| 519                 | Tapionaukio           | 478   

```

Lens can also work with datetime format that conform to Rust NaiveDateTime type. 
```
$ ./dynalens bike_data.csv --filter "Departure>2020-06-30T23:56:00"

Departure          | Departure station id| Departure station name| Duration(s)
-------------------|---------------------|-----------------------|------------
2020-06-30T23:59:20| 34                  | Kansallismuseo        | 887        
2020-06-30T23:58:21| 519                 | Tapionaukio           | 478  
```

## Installation
DynaLens is written in Rust. At the moment easiest way is to deploy DynaLens is to build it from source code with Rust tool chain. First clone this repo on your local host, and run
`cargo build --release`
Compiled application can be then the file _target/release/dynalens_.



