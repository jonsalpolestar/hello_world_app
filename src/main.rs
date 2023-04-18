fn main(){
    program();
 }

fn program<'life>() -> &'life str {
    println!("Rust says Hello World!!");
    for x in 1..11{ // 11 is not inclusive
       println!("x is {}",x);
       }
    let x:i32 = 7;
    fn_hello(x);

    let b = hello_string();
    println!("{}", b);

    return b;
}
 
 fn fn_hello(x:i32){
    println!("{} is the answer. Hello from function fn_hello() ", x);
 }

 fn hello_string<'life>() -> &'life str {
    let s = "hello world";
    return s;
}
 
 pub fn add(a: i32, b: i32) -> i32 {
     a + b
 }

 
 #[cfg(test)]
 mod tests {
     // Note this useful idiom: importing names from outer (for mod tests) scope.
     use super::*;
 
     #[test]
     fn test_program() {
         assert_eq!(program(), "hello world");
     }

     #[test]
     fn test_add() {
         assert_eq!(add(1, 2), 3);
     }
 
     #[test]
     fn test_hello_string() {
        assert_eq!(hello_string(), "hello world");
     }
 }
