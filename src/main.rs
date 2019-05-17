extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate checksum;

use std::io::prelude::*;
use std::fs;
use clap::{App, Arg};
use pest::Parser;

#[derive(Parser)]
#[grammar = "funcsharp.pest"]
struct FunCSharpParser;

fn process_variable(file: pest::iterators::Pairs<Rule>) -> (String, String, String) {    
    let mut var_type = String::default();
    let mut var_name = String::default();
    let mut var_value = String::default();

    for item in file {
        match item.as_rule() {
            Rule::type_reference => var_type += item.as_str(),
            Rule::identifier => var_name += item.as_str(),
            Rule::expression => var_value += item.as_str(),
            Rule::WHITESPACE | Rule::COMMENT => {},
            _ => println!("Not handled variable rule: {:?}\r\n {}", item.as_rule(), item.as_str()),  //unreachable!(),
        }
    }

    (var_type.trim().to_owned(), var_name.trim().to_owned(), var_value.trim().to_owned())
}

fn process_field(file: pest::iterators::Pairs<Rule>) -> (String, String, String) {    
    
    for item in file {
        match item.as_rule() {
            Rule::var_declaration => return process_variable(item.into_inner()),
            Rule::WHITESPACE | Rule::COMMENT | Rule::accessibility_decorator | Rule::const_decorator | Rule::semicolon => {},
            _ => println!("Not handled field rule: {:?}\r\n {}", item.as_rule(), item.as_str()),  //unreachable!(),
        }
    }
    unreachable!()
}

fn process_old(file: pest::iterators::Pairs<Rule>) -> (String, u32, String) {
    let mut generated = String::default();
    let mut version = String::default();
    let mut crc = 0u32;
    let mut keep = false;
    for item in file {
        match item.as_rule() {
            Rule::crc => if let Ok(cr) = u32::from_str_radix(item.as_str(), 16) { crc = cr; },
            Rule::version => { version = item.as_str().to_owned() },
            Rule::WHITESPACE | Rule::COMMENT | Rule::fill => 
                if keep { 
                    generated += item.as_str() 
                } else if item.as_str().contains('\n') {
                    keep = true;
                },
            Rule::region_end => keep = false,
            Rule::region_start | Rule::generated_marker => {},            
            _ => println!("Not handled field rule: {:?}\r\n {}", item.as_rule(), item.as_str()),  //unreachable!(),
        }
    }
    (generated, crc, version)
}

fn process_request(file: pest::iterators::Pairs<Rule>) -> String {    
    let mut output = String::default();

    let mut old_generated = String::default();
    let mut _given_old_version = String::default();
    let mut given_old_crc = 0u32;

    let mut class_name = String::default();
    let mut options = String::default();
    let mut fields = Vec::<(String, String, String)>::new();

    for item in file {        
        match item.as_rule() {
            Rule::gen_response => {
                ////output += item.as_str(); //remove old generated stuff from the output
                let (g, c, v) = process_old(item.into_inner());
                old_generated = g;
                _given_old_version = v;
                given_old_crc=c;
            }
            Rule::identifier => { 
                output += item.as_str();
                class_name += item.as_str();
            },
            Rule::options => { 
                output += item.as_str();
                options += item.as_str();
            },
            Rule::field_declaration => {
                output += item.as_str();
                fields.push(process_field(item.into_inner()));
            },
            Rule::WHITESPACE | 
            Rule::COMMENT |
            Rule::request_marker => output += item.as_str(),
            _ => println!("Not handled request rule: {:?}\r\n {}", item.as_rule(), item.as_str()),  //unreachable!(),
        }
    }

    let fields = fields;//make it readonly

    let mut generated = String::default();
    
    //property getters
    if options.contains('P') || options.contains('p') {
        for (type_name, name, _) in fields.to_owned() {
            generated += &format!("        public {0} {1}{2} {{ get {{ return this.{3}; }} }}\r\n", &type_name, &name[0..1].to_uppercase(), &name[1 as usize..], &name);
        }
    }

    //constructor
    if options.contains('C') || options.contains('c') {
        if options.contains('c') {
            generated += "\r\n        private ";
        } else {
            generated += "\r\n        public ";
        }
        generated += &class_name;
        generated += "(";
        let mut sep = "";
        for (type_name, name, value) in fields.to_owned() {
            generated += sep;
            sep = ", ";
            generated += &type_name;
            generated += " ";
            generated += &name;
            if value != "" {
                generated += " = "; 
                generated += &value;
            }
        }
        generated += ")\r\n        {\r\n";
        for (_, name, _) in fields.to_owned() {
            generated += "            this.";
            generated += &name; 
            generated += " = ";
            generated += &name;
            generated += ";\r\n";
        }
        generated += "        }\r\n";
    }

    //universal lens like WithXXX setters   //not good for value types: some ? need to be added manually
    if options.contains('w') {
        generated += "\r\n        public ";
        generated += &class_name;
        generated += " With(";
        let mut sep = "";
        for (type_name, name, _) in fields.to_owned() {
            generated += sep;
            sep = ", ";
            generated += &type_name;
            generated += " ";
            generated += &name;
            generated += " = null";
        }
        generated += ")\r\n        {\r\n            return new ";
        generated += &class_name;
        generated += "(\r\n";
        let mut i = 0;
        for (_, name, _) in fields.to_owned() {
            i += 1;
            generated += "                ";
            generated += &name;
            generated += ": ";
            generated += &name;
            generated += " ?? this.";        
            generated += &name; 
            generated += if i < fields.len() { ",\r\n" } else { ");\r\n" };
        }
        generated += "        }\r\n";
    }

    //lens like WithXXX setters
    if options.contains('W') {
        for (type_name, name, _) in fields.to_owned() {
            let n = name[0..1].to_uppercase().to_owned() + &name[1 as usize..];
            generated += &format!("        public {0} With{1}({2} {3})\r\n        {{\r\n            return new {0}(\r\n", &class_name, &n, &type_name, &name);
            let mut i = 0;
            for (_, nm, _) in fields.to_owned() {
                i += 1;
                generated += "                ";
                generated += &nm;
                generated += ": ";
                if nm != name {
                    generated += "this.";
                };
                generated += &nm;
                generated += if i < fields.len() { ",\r\n" } else { ");\r\n" };
            }
            generated += "        }\r\n";
        }
    }

    let new_crc = checksum::crc32::Crc32::new().checksum(generated.as_bytes());
   
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    output += &format!("#region #generated {:X} {}\r\n", new_crc, VERSION);
    output += &generated;
    output += "#endregion\r\n";

    let computed_old_crc = checksum::crc32::Crc32::new().checksum(old_generated.as_bytes());

    //if crc mismatch detected keep the old generated region as a comment block
    if given_old_crc != computed_old_crc {
        output += "\n/*\n";
        output += &old_generated;
        output += "\n*/\n";
    }
    
    output
} 

fn process_content(file: pest::iterators::Pairs<Rule>) -> String {    
    let mut output = String::default();

    for item in file {
        match item.as_rule() {
            Rule::gen_request => output += &process_request(item.into_inner()),
            Rule::WHITESPACE | 
            Rule::COMMENT | 
            Rule::region | 
            Rule::fill | 
            Rule::region_start |
            Rule::region_end => output += item.as_str(),            
            _ => println!("Not handled content rule: {:?}\r\n {}", item.as_rule(), item.as_str()),  //unreachable!(),
        }
    }

    output
}

fn process_file(file: pest::iterators::Pairs<Rule>) -> String {    
    let mut output = String::default();

    for item in file {
        match item.as_rule() {
            Rule::content => output += &process_content(item.into_inner()),
            Rule::WHITESPACE | Rule::COMMENT | Rule::EOI => output += item.as_str(),
            _ => println!("Not handled file rule: {:?}\r\n {}", item.as_rule(), item.as_str()),  //unreachable!(),
        }
    }

    output
}

fn parse(input: &str) -> Result<String, String> {
    match FunCSharpParser::parse(Rule::file, input) {
        Ok(result) => Ok(process_file(result)),
        Err(err) => Err(format_args!("{:?}", err).to_string()),
    }
}

fn run(input_filename: &str, output_filename: &str, backup_filename: Option<&str>) -> Result<(), String> {
    let input = fs::read_to_string(input_filename).expect("cannot read input file");
    let result = parse(&input)?;   

    if let Some(backup_filename) = backup_filename {
        if input == result {
            return Ok(());
        }            
        let mut bk = fs::File::create(backup_filename).expect("cannot open backup file");
        bk.write(input.as_bytes()).expect("cannot write backup file");        
    }
    let mut output = fs::File::create(output_filename).expect("cannot open output file");
    output
        .write(result.as_bytes())
        .expect("cannot write output file");

    Ok(())
}

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let matches = App::new("C++ sumtype")
        .version(VERSION)
        .about("Generates C# code: Constructor, With, Lens for readonly struct to get rid of typing boilerplate code.")
        .help(
r#"Generates C# boilerplate code from the fileds marked with a decorated region.

# Example usage in your code: 

    public struct Position2d
    {
#region func# Position2d CWwP
        public readonly double x;
        public readonly double y;
#endregion
        
        public static readonly Position2d Origo = new Position2d(0, 0);
    }

# Details:

#region func# <you class name> <options>
  <your fields / properties>
#endregion

# Options:

    C - public constructor generation for every fields
    c - private constructor generation for every fields
    w - With(...) generation for every fields at once
    W - WithXXX(.) generation for every fields separately
    P - property getter generation for every fields"#)

        .author("Tibor P")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file to use (may be the same as the INPUT)")
            .required(true)
            .index(2))
        .arg(Arg::with_name("BACKUP")
            .help("Sets the backup file to use (when the OUTPUT is the same as the INPUT)")
            .required(false)
            .index(3))
        .get_matches();

    if let Some(input_filename) = matches.value_of("INPUT") {
        if let Some(output_filename) = matches.value_of("OUTPUT") {
            if let Err(error) = run(input_filename, output_filename, matches.value_of("BACKUP")) {
                println!("ERROR: {:?}", error);
            } else {
                println!("Successfully written: {}", output_filename);
            }
        }
    }
}
