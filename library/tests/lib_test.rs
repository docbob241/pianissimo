#[cfg(test)]
mod tests {
    use oof::process_bytes;
    use std::{fs::File, io::Read};

    #[test]
    fn test_process_bytes() {
        // Read the contents of the file into a Vec<u8>
        let mut file_data = Vec::new();
        let mut file = File::open("test.wav").unwrap();
        file.read_to_end(&mut file_data).unwrap();

        let result = process_bytes(&file_data);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), 220370.8);
    }
}
