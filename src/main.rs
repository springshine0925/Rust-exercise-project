// fn main() {
//     let action = std::env::args().nth(1).expect("Please secify an action");
//     let item=std::env::args().nth(2).expect("Please specify an item");
//     println!("{:?}and{:?}", action, item);
// }
use std::collections::HashMap;

struct Todo{
    map: HashMap<String, bool>;
    fn insert(&mut self, key:String){
         self.map.insert(key,true);
    }
}
