fn main()
{
    // for loop
    for x in 1..11{
        if x==5{
           break;
        }
        
        println!("{}",x);
    }
    //while loop
    let mut y=10;
    while y>0{
        println!("{}",y);
        y-=1;
    }
}
