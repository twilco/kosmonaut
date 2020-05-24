use crate::new_cmd;

#[cfg(test)]
mod tests {
    use crate::new_cmd;

    #[test]
    fn rainbow_divs() {
        new_cmd().succeeds();
    }
}