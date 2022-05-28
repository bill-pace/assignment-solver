pub struct View {
    infile: Option<String>,
    outfile: Option<String>
}

impl View {
    pub fn get_infile_name(&self) -> Result<String, std::io::Error> {
        match &self.infile {
            Some(s) => Ok(s.clone()),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound,
                                            "You must select an input file!"))
        }
    }

    pub fn get_outfile_name(&self) -> Result<String, std::io::Error> {
        match &self.outfile {
            Some(s) => Ok(s.clone()),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound,
                                            "You must select an output file!"))
        }
    }
}
