fn main()
{
    let  a=32;
    let  b=34;
    add(a,b);
    let mut c=35;
    modifyvalue(&mut c);
    println!("After modifying state of c");
    println!(" c = {}",c);
    let mut e =10;
    let mut f=15;
    println!("After swapping two numbers");
    swap(&mut e,&mut f);
    println!(" e = {}  f =  {} ",e,f);
    let sub1=sub(a,b);
    println!("sub = {} ",sub1);
    let name:String = String::from("Aniruddha");
    let surname:String = String::from("Kulkarni");
    displayname(name,surname);



   
}
fn add( x:i32, y:i32)//call by value
{
    println!("Addition is : {} ",x+y);
}
fn modifyvalue(value:&mut i32)//call by reference
{
    *value+=5;

}
fn swap(n1:&mut i32,n2:&mut i32)//swapping two numbers
{
    let t=*n1;
    *n1=*n2;
    *n2=t;
}
fn sub(x:i32 ,y:i32) ->i32 //mentioning return type of function
{
    return x-y;

}
fn displayname(Name:String,Surname:String)//string object as argument
{
    let c = Name   + &Surname;
    println!("Name : {}" , c);
}

