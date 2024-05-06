use std::io;
fn main() {
   let a=[1,2,3,4,5];
   println!("please enter an array index.");
   let mut index =String::new();
   io::stdin().read_line(&mut index).expect("Failed to read line");
   let index:usize=index.trim().parse().expect("index entered was not a number");
   let element=a[index];
   println!("The value of the element at index {index} is : {element}");

}
// use std::collections::HashMap;

// struct Todo{
//     map: HashMap<String, bool>;
//     fn insert(&mut self, key:String){
//          self.map.insert(key,true);
//     }
// }
