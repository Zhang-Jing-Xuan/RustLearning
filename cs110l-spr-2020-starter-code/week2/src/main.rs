fn main(){
    let mut s:String=String::from("Hello world!");
    let s1=& mut s;
    println!("{}",s1);
    println!("{}",s);
    
    
    // println!("{},{}",s1,s);
}