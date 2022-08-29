
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let missiles: i32 = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    let missile_launcher_id: String;

    println!("Firing {} of my {} missiles...", ready, missiles);

    println!("{} missiles left", missiles - ready);
    // READY_AMOUNT = READY_AMOUNT - 1;
}
