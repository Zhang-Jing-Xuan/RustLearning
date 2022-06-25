fn main(){
    let n:i32=1;
    println!("{}",n);
    let mut m:i32=0;
    m=m+1;
    println!("{}",m);
    let s:&str="Hello World!";
    println!("{}",s);
    let s:String=String::from("Hello World again!");
    println!("{}",s);
    let mut v:Vec<i32>=Vec::new();
    v.push(2);
    v.push(3);
    let mut arr:[i32;4]=[0,2,4,8];
    arr[0]=-2;
    println!{"{}",arr[0]+arr[3]};
    for i in v.iter(){
        println!("{}",i)
    }
    let mut i:i32=0;
    while i<20 {
        i+=1;
    }
    let mut i=0;
    loop{
        i+=1;
        if i>10{
            break;
        }
    }
}

fn sum(a:i32,b:i32)->i32{
    a+b
}
fn fib(n:i32)->i32{
    if n<=1{n}else{fib(n-1)+fib(n-2)}
}