#![deny(unused_must_use)]

use wallee::wallee;

fn main() -> wallee::Result<()> {
    if true {
        // meant to write bail!
        wallee!("it failed");
    }
    Ok(())
}
