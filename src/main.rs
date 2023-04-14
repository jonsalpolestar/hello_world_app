fn main(){
    println!("Rust says Hello World!!");
    for x in 1..11{ // 11 is not inclusive
       println!("x is {}",x);
   }
   let x:i32 = 7;
    fn_hello(x);
 }
 
 
 fn fn_hello(x:i32){
    println!("{} is the answer. Hello from function fn_hello() ", x);
 }
 
 pub fn add(a: i32, b: i32) -> i32 {
     a + b
 }
 
 // This is a really bad adding function, its purpose is to fail in this
 // example.
 #[allow(dead_code)]
 fn bad_add(a: i32, b: i32) -> i32 {
     a - b
 }
 
 #[cfg(test)]
 mod tests {
     // Note this useful idiom: importing names from outer (for mod tests) scope.
     use super::*;
 
     #[test]
     fn test_add() {
         assert_eq!(add(1, 2), 3);
     }
 
     #[test]
     fn test_bad_add() {
         // This assert would fire and test will fail.
         // Please note, that private functions can be tested too!
         assert_eq!(bad_add(1, 2), 3);
     }
 }