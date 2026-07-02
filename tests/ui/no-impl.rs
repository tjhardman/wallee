use wallee::wallee;

#[derive(Debug)]
struct Error;

fn main() {
    let _ = wallee!(Error);
}
