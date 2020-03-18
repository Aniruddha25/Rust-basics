fn main()
{
    let a:(i32,String,f64)=(23,"AMK".to_string(),24.25);
    println!("{:?}",a);
    println!("float is {:?}",a.2);//for accessing 3rd element of tuple
    tuplefunc(a);
}
fn tuplefunc(y:(i32,String,f64))//passing tuple to function
{
    println!("Tuple : {:?} ",y);
}