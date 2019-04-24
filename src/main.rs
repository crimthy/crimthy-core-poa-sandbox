mod block;
mod broadcast;
mod utils;
mod connection;

use utils::replace;
use broadcast::broadcast_block;

fn main() {
    let mut peers = vec![String::from("ab"),String::from("abc"), String::from("abd")];
    broadcast_block(&mut peers);
    //let a = replace(&mut peers, &String::from("abc"), String::from("gga")).expect("Can't replace host address");
    //println!("{}", a);
    println!("{:?}",peers);
}
 