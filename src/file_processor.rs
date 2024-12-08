use std::fs;

const HTML_EXT: &str = "html";

#[derive(Clone)]
pub struct FileProcessor {
    pub files: Vec<String>,
}

impl FileProcessor {
    pub fn new(dir_path: String) -> Self {
        let mut file_processor = Self { files: vec![] };
        file_processor.scan_dir(&dir_path);
        file_processor
    }
    fn scan_dir(&mut self, path: &str) {
        let paths = fs::read_dir(path).unwrap();

        for path in paths {
            let obj = path.expect("Error reading file");

            if obj.file_type().expect("Error reading file type").is_file() {
                let file_name =
                    String::from(obj.file_name().to_str().expect("Can't read file name"));
                let raw_file_name = file_name.split('.').next().expect("Invalid file");
                let ext = file_name.split('.').last().expect("Invalid file");
                if ext == HTML_EXT {
                    self.files.push(raw_file_name.to_string());
                }
            }
        }
        self.files.push(String::from(""));
    }
}
