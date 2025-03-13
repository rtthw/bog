


use bog_types::*;
use env::*;



fn main() -> Result<()> {
    let mut conn = connect()?;
    let reply = conn.request(Request {
        code: [0, 0, 0, 0],
        sender: conn.id(),
        data: RequestData::CreateWindow,
    })?;

    println!("Got reply: {:?}", reply);

    Ok(())
}
