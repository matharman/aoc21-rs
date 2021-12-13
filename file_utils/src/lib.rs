use std::fs::OpenOptions;
use std::io::{self, BufRead};

pub struct LineReader<'a> {
    name: &'a str,
    inner: io::Lines<io::BufReader<std::fs::File>>,
}

pub type Result<T> = io::Result<T>;

impl<'a> LineReader<'a> {
    pub fn open(path: &'a str) -> Result<Self> {
        let f = OpenOptions::new().read(true).open(path)?;
        let inner = std::io::BufReader::new(f).lines();
        Ok(Self { name: path, inner })
    }

    pub fn dup(&self) -> Result<Self> {
        LineReader::open(self.name)
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Iterator for LineReader<'a> {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
