const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    /*
    let mut missiles: i32;
    let ready: i32;

    missiles = STARTING_MISSILES;
    ready = READY_AMOUNT;
    */

    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
}
