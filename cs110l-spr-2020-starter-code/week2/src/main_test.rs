fn change_it_up(s: &mut String) {
    *s = "goodbye".to_string();
}
fn make_it_plural(word: &mut String) {
    word.push('s');
}
fn let_me_see(s: &String) {
    println!("{}", s);
}
fn let_me_see2(s: String) {
    println!("{}", s);
}
fn main() {
    let mut s = "hello".to_string();
    println!("{}", s);
    change_it_up(&mut s);
    let_me_see(&s);
    make_it_plural(&mut s);
    let_me_see(&s);
    // let's make it even more plural
    s.push('s'); // does this seem strange?
    let_me_see(&s);

    let s1:String="Hello".to_string();
    let_me_see2(s1);
    // println!("{}", s1);

}