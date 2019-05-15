extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use clap::{Arg, App};
use std::io::prelude::*;
use pest::Parser;
//use std::collections::HashSet;

#[derive(Parser)]
#[grammar = "funcsharp.pest"]
struct FunCSharpParser;
/*
fn process_sumtype_item(sumtype_item: pest::iterators::Pairs<Rule>) -> (String, String, String)
{   
    let mut output = String::default();
    let mut item_name = String::default();
    let mut item_type = String::default();
    
    for item in sumtype_item {
        //println!("***{:?}***\n {}", item.as_rule(), item.as_str());
        match item.as_rule() {
            Rule::WHITESPACE => {
                output += item.as_str();
            },
            Rule::COMMENT => {
                output += item.as_str();
            },                        
            Rule::simple_sumtype_item => {
                item_name = item.as_str().to_owned();
                output = output + "_" + item.as_str();
            },
            Rule::typed_sumtype_item => {
                for item in item.into_inner()
                {
                    //println!("****{:?}****\n {}", item.as_rule(), item.as_str());
                    match item.as_rule() {
                        Rule::identifier => {
                            item_name = item.as_str().to_owned();
                            output = output + "_" + item.as_str();
                        },
                        Rule::type_reference => {
                            item_type = item.as_str().to_owned();
                            output = output + " /* (" + item.as_str() + ") */";
                        }
                        _ => unreachable!(),
                    }
                }
            },
            Rule::semicolon => {
                output += ",";
            },
            _ => unreachable!(),
        }                
    }

    ("\t".to_owned() + &output, item_name, item_type)
}

fn process_sumtype_definition(sumtype: pest::iterators::Pairs<Rule>) -> String
{
    let mut output = String::default();
    let mut sumtype_name = String::default();
    let mut items = Vec::<(String, String)>::new();
    let mut done = HashSet::<String>::new();

    for item in sumtype { 
        //println!("**{:?}**\n {}", item.as_rule(), item.as_str());

        match item.as_rule() {
            Rule::WHITESPACE => {
                output += item.as_str();
            },
            Rule::COMMENT => {
                output += item.as_str();
            },
            Rule::curly_open => {
                output += 
r#"{
public:
    enum Tags
    {"#;
            },            
            Rule::type_name => {
                output = output + "class " + item.as_str();
                sumtype_name = item.as_str().to_owned();
            },
            Rule::sumtype_item => {
                let (o, n, t) = process_sumtype_item(item.into_inner());
                output += &o;
                items.push((n,t));
            },
            Rule::curly_close => {                
                output += 
r#"    };

private:
    Tags _tag;

    union
    {
"#; 
                for pair in items.iter() {
                    if pair.1.is_empty() { 
                        continue; 
                    }
                    output = output + "\t\t" + &pair.1 + " " + &pair.0 + ";\n";
                }
                output = output + 
r#"    };

    "# + &sumtype_name + r#"(Tags tag) : 
        _tag(tag)
    {
    }
"#;
                for pair in items.iter() {
                    if pair.1.is_empty() { 
                        continue; 
                    }
                    let conflict = done.contains(&pair.1);
                    done.insert(pair.1.to_owned());

                    if conflict {
                        output += r#"
    //This overload already exist above.
    /*"#;
                    }

                    output = output + 
r#"
    "# + &sumtype_name + r#"(Tags tag, "# + &pair.1 + r#" value) : 
        _tag(tag),
        _"# + &pair.0 + r#"(value)
    {
    }
"#;                  
                    if conflict {
                        output += "    */\n";
                    }
                }

                output = output + 
r#"
public:
    ~"# + &sumtype_name + r#"()
    {
        //call only the required destructor explicitly
        switch (_tag)
        {
"#;
                for pair in items.iter() {
                    if pair.1.is_empty() { 
                        continue; 
                    }
                    output = output + 
"        case " + &pair.0 + ": _" + &pair.0 + ".~" + &pair.1 + "(); break;\n";
                }
                output = output + 
r#"        }
    }

    Tags tag() { return _tag; };
    
"#;
                for pair in items.iter() {
                    
                    output = output + 
"    static " + &sumtype_name + " " + &pair.0 + 
                    &if pair.1.is_empty() { 
                        r#"() 
    {
        return "#.to_owned() + &sumtype_name + "(" + &pair.0 
                    } else {
                        "(".to_owned() + &pair.1 + r#" value) 
    {
        return "# + &sumtype_name + "(" + &pair.0 + ", value" 
                    } + r#");
    }
    
"#
                }

                //...

                output += "\n}";
            },
            Rule::semicolon => {
                output += ";";
            },
            _ => unreachable!(),
        }        
    }

    output
}
*/

fn process_file(file: pest::iterators::Pairs<Rule>) -> String
{
    return format!("{:?}\n", &file);

    let mut output = String::default();

    for item in file { 
        //println!("*{:?}*\n {}", item.as_rule(), item.as_str());

        match item.as_rule() {
            // Rule::fill => {
            //     output += item.as_str();
            // },            
            // Rule::gen_request => {
            //     output += item.as_str();//&process_sumtype_definition(item.into_inner());
            // },
            // Rule::COMMENT => {
            //     output += item.as_str();
            // },
            Rule::WHITESPACE => {
                output += item.as_str();
            },
            // Rule::tail => {
            //     output += item.as_str();
            // },
            Rule::EOI => {                
            },
            _ => unreachable!(),
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
    let result =  
    {
        let input = fs::read_to_string(input_filename).expect("cannot read input file");    
        parse(&input)?
    };
    let mut output = fs::File::create(output_filename).expect("cannot open output file");
    output.write(result.as_bytes()).expect("cannot write output file");
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

    if let Some (input_filename) = matches.value_of("INPUT") { 
        if let Some (output_filename) = matches.value_of("OUTPUT") {             
            if let Err(error) = run(input_filename, output_filename) {
                println!("ERROR: {:?}", error);
            } else {
                println!("Successfully written: {}", output_filename);
            }
        }
    }
}
