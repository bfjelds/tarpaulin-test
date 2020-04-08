
fn main() {
    println!("main");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_file_access() {
        let relative_path = "../files/contents.txt";
        let file_path = std::fs::canonicalize(std::path::PathBuf::from(&relative_path))
            .expect(&format!("unable to find test file: {}", &relative_path))
            .to_str()
            .expect("unable to convert mpeg PathBuf to &str")
            .to_string();
        let _contents = std::fs::read_to_string(&file_path)
            .expect(&format!("unable to read test file: {}", &file_path));
        assert!(true);
    }
}