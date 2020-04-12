extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let mut measurement: bool = false;
    let file = File::open("output_vegvesen.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let name = name.local_name;
                if name == "siteMeasurements" {
                    measurement = true
                }
                if measurement {
                    print!("{}+{}", indent(depth), name);
                    for attribute in attributes {
                        print!(":{}", attribute.value);
                    }
                    println!();
                    depth += 1;
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                if measurement {
                    println!("{}{}", indent(depth), s);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if measurement {
                    let _ = name.local_name;
                    //println!("{}-{}", indent(depth), name);
                    if depth > 0 {
                        depth -= 1;
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
