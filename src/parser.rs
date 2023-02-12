use crate::tasks::SubOperator;



// Split where clause, e.g. Duration > 30 will return column name for Duration column, condition and comparison value
fn split_tool<'a>(filter_clause: &'a str, operator: &'a str) -> Vec<&'a str>{

    let mut split: Vec<&str> = filter_clause.split(operator).collect();

    if split.len() != 2{
        panic!["Where clause is not understood: {:?}. Missing elements", split];
    }

    split = split.iter().map(|x| x.trim()).collect();
   
    split
}


// Parse where task parameters from user input. Return operator, column name and value
pub fn filter_parser<'a>(filter_clause: &'a str) -> (SubOperator, &'a str, &'a str){

    let split: Vec<&str>;
    let operator: SubOperator;

    if filter_clause.contains("<="){
        split = split_tool(filter_clause, "<=");
        operator = SubOperator::LessOrEqualThan;
    }
    else if filter_clause.contains("<"){
        split = split_tool(filter_clause, "<");
        operator = SubOperator::LessThan;
    }
    else if filter_clause.contains(">="){
        split = split_tool(filter_clause, ">=");
        operator = SubOperator::GreaterOrEqualThan;
    }
    else if filter_clause.contains(">"){
        split =  split_tool(filter_clause, ">");
        operator = SubOperator::GreaterThan;
    }
    else if filter_clause.contains("="){
        split =  split_tool(filter_clause, "=");
        operator = SubOperator::EqualTo;
    }
    else if filter_clause.contains("=="){
        split =  split_tool(filter_clause, "==");
        operator = SubOperator::EqualTo;
    }
    else if filter_clause.contains("!="){
        split =  split_tool(filter_clause, "!=");
        operator = SubOperator::NotEqualTo;
    }
    else{
        panic!["Where clause is not understood: {:?}. Cannot find <, >, <=, >= or =", filter_clause];
    }

    (operator, split[0], split[1])
}

#[cfg(test)]
mod tests {

    use super::*;
    

#[test]
fn test_split_tool(){

    let filter_clause = "Duration > 30";

    let split = split_tool(filter_clause, ">"); 
    assert_eq!(split[0], "Duration");
    assert_eq!(split[1], "30");
}

#[should_panic]
#[test]
fn test_failing_split_tool(){

    let filter_clause = "Jution  30";
    
    let _  = split_tool(filter_clause, ">"); 
}
}