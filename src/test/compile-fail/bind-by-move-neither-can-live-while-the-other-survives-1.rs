struct X { x: () }

impl X : Drop {
    fn finalize() {
        error!("destructor runs");
    }
}

fn main() {
    let x = Some(X { x: () });
    match move x {
        Some(ref _y @ move _z) => { }, //~ ERROR cannot bind by-move and by-ref in the same pattern
        None => fail
    }
}
