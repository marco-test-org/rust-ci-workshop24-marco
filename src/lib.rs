pub fn add() {
    println!("hello");
}

#[cfg(test)]
mod tests {
    #[test]
    fn small_test() {
        println!("hello world");
    }
}
