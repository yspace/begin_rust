pub fn action_ref_as_param(){
 let s1 = String::from("hello");
    let len = calc_len(&s1) ;

    println!("The length of '{}' is {} ", s1, len);
}
fn calc_len(s: &String) -> usize {
    s.len()
}