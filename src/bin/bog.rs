//! Bog Runtime



use bog::window;
use env::*;



fn main() -> Result<()> {
    let env = match Env::connect() {
        Ok(env) => env,
        Err(err) => match err {
            Error::Connection(ConnectError::NotRunning) => {
                // Environment isn't running, start it.
                Env {}
            }
            Error::Connection(ConnectError::Other(msg)) => {
                // FATAL: Environment cannot be connected to for reasons outside our control.
                panic!("FATAL: Could not connect to environment: {msg}");
            }
        }
    };

    let window = window::create(&env)?;

    println!("INFO: Created window {window:?}");

    Ok(())
}
