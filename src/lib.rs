pub fn add() {
    let var1 = Some(1);
    if let Some(num) = var1 {
        println!("{num}")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn small_test() {
        panic!("hello world");
    }
}
