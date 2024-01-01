fn mutability() {
    let x = 5;
    println!("x = {x}");

    // cannot mutate immutable variable `x`
    // x = 6;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutability() {
        mutability()
    }
}
