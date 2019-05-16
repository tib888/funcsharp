extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use clap::{App, Arg};
use pest::Parser;

use std::io::prelude::*;

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

fn process_request(file: pest::iterators::Pairs<Rule>) -> String {    
    let mut output = String::default();

    let mut class_name = String::default();
    let mut options = String::default();
    let mut fields = Vec::<(String, String, String)>::new();

    for item in file {        
        match item.as_rule() {
            Rule::gen_response => {},    //remove old generated stuff from the output
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

    output += "#region #generated\r\n";
    
    //property getters
    if options.contains('P') || options.contains('p') {
        for (type_name, name, _) in fields.to_owned() {
            output += &format!("        public {0} {1}{2} {{ get {{ return this.{3}; }} }}\r\n", &type_name, &name[0..1].to_uppercase(), &name[1 as usize..], &name);
        }
    }

    //constructor
    if options.contains('C') || options.contains('c') {
        if options.contains('c') {
            output += "\r\n        private ";
        } else {
            output += "\r\n        public ";
        }
        output += &class_name;
        output += "(";
        let mut sep = "";
        for (type_name, name, value) in fields.to_owned() {
            output += sep;
            sep = ", ";
            output += &type_name;
            output += " ";
            output += &name;
            if value != "" {
                output += " = "; 
                output += &value;
            }
        }
        output += ")\r\n        {\r\n";
        for (_, name, _) in fields.to_owned() {
            output += "            this.";
            output += &name; 
            output += " = ";
            output += &name;
            output += ";\r\n";
        }
        output += "        }\r\n";
    }

    //universal lens like WithXXX setters   //not good for value types: some ? need to be added manually
    if options.contains('W') || options.contains('w') {
        output += "\r\n        public ";
        output += &class_name;
        output += " With(";
        let mut sep = "";
        for (type_name, name, _) in fields.to_owned() {
            output += sep;
            sep = ", ";
            output += &type_name;
            output += " ";
            output += &name;
            output += " = null";
        }
        output += ")\r\n        {\r\n            return new ";
        output += &class_name;
        output += "(\r\n";
        let mut i = 0;
        for (_, name, _) in fields.to_owned() {
            i += 1;
            output += "                ";        
            output += &name;
            output += " ?? this.";        
            output += &name; 
            output += if i < fields.len() { ",\r\n" } else { ");\r\n" };
        }
        output += "        }\r\n";
    }

    //lens like WithXXX setters
    if options.contains('L') || options.contains('l') {
        for (type_name, name, _) in fields.to_owned() {
            let n = name[0..1].to_uppercase().to_owned() + &name[1 as usize..];
            output += &format!("        public {0} With{1}({2} {3})\r\n        {{\r\n            return new {0}(\r\n", &class_name, &n, &type_name, &name);
            let mut i = 0;
            for (_, nm, _) in fields.to_owned() {
                i += 1;
                output += if nm != name {
                    "                this."
                } else {
                    "                "
                };
                output += &nm;
                output += if i < fields.len() { ",\r\n" } else { ");\r\n" };
            }
            output += "        }\r\n";
        }
    }

    output += "#endregion\r\n";
    
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

fn run(input_filename: &str, output_filename: &str) -> Result<(), String> {
    let input = fs::read_to_string(input_filename).expect("cannot read input file");
    let result = parse(&input)?;   

    if input_filename == output_filename {
        if input == result {
            return Ok(());
        }            
        let mut bk = fs::File::create(input_filename.to_owned() + ".bk").expect("cannot open backup file");
        bk.write(input.as_bytes()).expect("cannot write .bk file");        
    }
    let mut output = fs::File::create(output_filename).expect("cannot open output file");
    output
        .write(result.as_bytes())
        .expect("cannot write output file");

    Ok(())
}

fn main() {
    let matches = App::new("C++ sumtype")
        .version("0.1")
        .about("Generates C# code: Constructor, With, Lens for readonly struct to get rid of typing boilerplate code.")
        .author("Tibor P")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file to use //TODO (may be the same as the INPUT)")
            .required(true)
            .index(2))
        .get_matches();

    if let Some(input_filename) = matches.value_of("INPUT") {
        if let Some(output_filename) = matches.value_of("OUTPUT") {
            if let Err(error) = run(input_filename, output_filename) {
                println!("ERROR: {:?}", error);
            } else {
                println!("Successfully written: {}", output_filename);
            }
        }
    }
}
