fn main() {
   let filter= FilterCondition{
    number : 2
   };

   let numbers = vec![1,2,3,4,2,5];

   let result = custom_filter(numbers, &filter);

   for number in result.iter(){
        println!("{}",number);
   }
}
struct FilterCondition{
    number: i32
}

impl FilterCondition{
    fn is_match(&self,&_number: &i32) -> bool{
         _number == self.number
    }
}

fn custom_filter(numbers: Vec<i32> , filter :&FilterCondition) ->  Vec<i32>{

    let mut new_collection= vec![];
    
    for number in numbers.iter() {
        if(filter.is_match(&number)){
            new_collection.push(*number);
        }
    }

    new_collection
}
