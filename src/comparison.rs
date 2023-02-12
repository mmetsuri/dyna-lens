use crate::tasks::SubOperator;

pub trait Comparison<T: PartialOrd>{

    fn comparison(&self, record: &T, comparison_type: SubOperator) -> bool;
    
}

impl<T:PartialOrd> Comparison<T> for T{
    

    fn comparison(&self, record: &T, comparison_type: SubOperator) -> bool{

        match comparison_type{
            SubOperator::GreaterOrEqualThan => {if record >= self{ return true }},
            SubOperator::GreaterThan => {if record > self {return true}},
            SubOperator::LessOrEqualThan => {if record <= self{ return true }},
            SubOperator::LessThan => {if record < self{ return true }},
            SubOperator::EqualTo => {if record == self{ return true }},
            SubOperator::NotEqualTo => {if record != self{ return true }}
        }
        false
    }

}
