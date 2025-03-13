


use std::time::Instant;

use bog_types::*;
use env::*;


/*

NOTES:
    - As of 3/13/25, on my Linux workstation:
        DEBUG: 1000 requests => 1.5 ms
        RELEASE: 1000 requests => 0.5 ms

*/

const REQ_COUNT: usize = 1000;



fn main() -> Result<()> {
    let start = Instant::now();

    let mut conn = connect()?;

    println!("\n\tPerforming {} requests...", REQ_COUNT);

    for _ in 0..REQ_COUNT {
        let reply = conn.create_window("Something")?;
        if !reply.success {
            println!("ERROR");
        }
    }

    let time_taken = Instant::now().duration_since(start);

    println!("\n\tTook {}ns, or {} ms", time_taken.as_nanos(), time_taken.as_millis());

    Ok(())
}
