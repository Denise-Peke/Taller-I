use std::fs::File;
use std::io::{self, BufRead};


pub struct Reader {
    file_name: String,
    search_string: String,
}

impl Reader {
  
    pub fn new(file_name: String, search_string: String) -> Self {
        Reader {
            file_name,
            search_string,
        }
    }

    pub fn read_and_search(&self) -> io::Result<()> {
       
        let file = File::open(&self.file_name)?;
        let reader = io::BufReader::new(file);

    
        for (_line_number, line) in reader.lines().enumerate() {
            let line = line?;
            if let Some(_pos) = self.find_substring(&line) {
                println!("{}", line);
            }
        }
        Ok(())
    }

    
    fn find_substring(&self, s: &str) -> Option<usize> {
        if let Some(pos) = s.find(&self.search_string) {
            Some(pos)
        } else {
            None
        }
    }
}
