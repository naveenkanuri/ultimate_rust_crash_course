
const STARTING_MISSILES: i32 = 8;
const READY_MISSILES: i32 = 2;

fn main() {
    println!("Hello, world!");
    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_MISSILES;
    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_MISSILES);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles-= ready;
    println!("Firing {} of my {} missiles...", ready, missiles-ready);
}
