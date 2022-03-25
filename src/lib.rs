//! run with `cargo test -j 1`

#[cfg(test)]
mod tests {
    #[test]
    fn set_a() {
        std::env::set_var("A", "true");
        assert!(std::env::var("B").is_err());
    }

    #[test]
    fn set_b() {
        std::env::set_var("B", "true");
        assert!(std::env::var("A").is_err());
    }
}
