//String literals are also known as string slices.
fn main(){
   let name:&str = "aniruddha";//string literals
    let surname:&str = "kulkarni";
   /* let name = "aniruddha";
    let surname = "kulkarni";*/

    println!("My Name is {} {}  ",name,surname);
    println!("{}",name.len());
    /*string objects 
     String object is allocated in the heap.
    Method 1 */
    let mut description = String::new();//empty string
    println!("{} ",description);
    //method 2
    let desc = String::from("Hey this is Aniruddha");
    println!(" { }",desc);
    let a =3;
    let b = a.to_string();//to convert to string datatype
    println!("{}",b);
    description.push_str("CDAC");//To push content to empty string
    println!("{}",description);
    let description = description.replace("CDAC","DAC");//replacing string content
    println!("{}",description);
    /*Individual characters in a string can be accessed using the chars method.*/
    for i in description.chars(){
        println!("{}",i);
    }
    //Concatenation of two strings
   let a= "Ani".to_string();
   let b ="ruddha".to_string();
   let c = a + &b;
   println!("{}",c);
  


}