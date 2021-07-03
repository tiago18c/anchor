use anchor_syn::idl::{Idl, IdlField, IdlTypeDefinitionTy};
use anyhow::{anyhow, Context, Result};
use heck::{CamelCase, SnakeCase};

pub fn generate(idl: &Idl, out: &str) -> Result<()> {
    // todo: generate a template string and write to the output file


    let mut code = format!("using Solnet.Rpc;\nnamespace {}\n{{\n\n // Type segments\n", idl.name.to_camel_case());


    for ty  in &idl.accounts {
        let accname = ty.name.as_str();

        match ty.ty {
            IdlTypeDefinitionTy::Struct { fields } => {
                code.push_str(&format!("\tpublic class {}\n\t{{\n", accname));
                code.push_str(&generateClassFields(&fields));
            },
            IdlTypeDefinitionTy::Enum { variants } => {
                code.push_str(&format!("\tpublic enum {}\n\t{{\n", accname));
            }
        }

        
        

        
        
        code = code + "\t}\n";
    }

    code = code + "}\n";


    println!("please be patient, currently learning rust and generating C# from IDL\n\n");


    println!("{}", code);
    Ok(())
}


fn generateClassFields(fields :&Vec<IdlField>) -> String {
    for field in fields {
        match field.ty {
            
        }
    }
    "".to_owned()
}