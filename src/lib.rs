pub fn template_function(x: i32) -> i32 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(template_function(2), 4);
    }
}
