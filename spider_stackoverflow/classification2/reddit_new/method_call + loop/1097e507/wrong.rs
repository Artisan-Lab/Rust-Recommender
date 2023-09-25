use std::io::{BufReader, BufWriter, BufRead};
use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

struct MdParser<R: BufRead> {
    reader: R,
    flags: MdFlags,
    rendered_out: String,
    line_start_index: u8, // First line character after the tag
}
impl<R: BufRead> MdParser<R> {
    fn new(reader: R) -> Result<MdParser<R>, Box<dyn std::error::Error>> {
        Ok( MdParser {
            reader,
            flags: MdFlags::new(),
            rendered_out: String::new(),
            line_start_index: 0,
        })
    }

    fn parse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for line in  self.reader.lines(){
            let l = line?;

            // Get the line starting tag
            if l.starts_with("###") {
                self.flags.h3 = true;
                self.line_start_index = 3;
            } else if l.starts_with("##") {
                self.flags.h2 = true;
                self.line_start_index = 2;
            } else if l.starts_with("#") {
                self.flags.h1 = true;
                self.line_start_index = 1;
            } else {
                self.flags.p = true;
            }

            // Insert the line starting tag to the output string
            if self.flags.p {
                self.rendered_out.push_str("<p>");
            } else if self.flags.h1 {
                self.rendered_out.push_str("<h1>");
            } else if self.flags.h2 {
                self.rendered_out.push_str("<h2>");
            } else if self.flags.h3 {
                self.rendered_out.push_str("<h3>");
            }

            // Insert the actual line content
            self.rendered_out.push_str(&l[self.line_start_index as usize ..]);

            // Insert ending tag
            // Insert the tag to the output string
            if self.flags.p {
                self.rendered_out.push_str("</p>\n");
                self.flags.p = false;
            } else if self.flags.h1 {
                self.rendered_out.push_str("</h1>\n");
                self.flags.h1 = false;
            } else if self.flags.h2 {
                self.rendered_out.push_str("</h2>\n");
                self.flags.h2 = false;
            } else if self.flags.h3 {
                self.rendered_out.push_str("</h3>\n");
                self.flags.h3 = false;
            } else {
                panic!("Error no closing tag on the line {}", &l);
            }
        }
        Ok(())
    }
}

pub struct MdFlags {
    pub p: bool,
    pub h1: bool,
    pub h2: bool,
    pub h3: bool
}
impl MdFlags {
    fn new() -> MdFlags {
        MdFlags {
            p: false,
            h1: false,
            h2: false,
            h3: false,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("test.md")?;
    let reader = BufReader::new(f);
    let mut m = MdParser::new(reader)?;
    m.parse();
    println!("{}", m.rendered_out);

    Ok(())
}