fn main(){
    let a=2;
    if a>2{
        println!("{}  is greater than 2",a);
     }
     else if a==2{
         println!("{} = 2",a);
     }
     else
     {
         println!("{} is less than 2",a);
     }
     //Match Statement
     /*The match statement checks if a current value is matching from a list of values, this is very much similar to the switch statement in C language. */
     let state_code = "KA";
     let state = match state_code {
        "MH" => {println!("Found match for MH"); "Maharashtra"},
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown"
     };   
     println!("{}",state);
}