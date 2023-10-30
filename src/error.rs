pub struct Report {
    line: usize,
    location: String,
    msg: String,
}

impl Report {
    pub fn new(line: usize, location: String, msg: String) -> Self {
        Report {
            line,
            location,
            msg,
        }
    }

    pub fn report(&self) {
        println!("[line {}] Error {}: {}", self.line, self.location, self.msg);
    }
}
