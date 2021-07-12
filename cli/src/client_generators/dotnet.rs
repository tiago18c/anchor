use anchor_syn::idl::{Idl, IdlEnumVariant, IdlField, IdlType, IdlTypeDefinition, IdlTypeDefinitionTy};
use anyhow::{anyhow, Result};
use heck::{CamelCase, SnakeCase};
use tera::{Tera, Context};

pub fn generate(idl: &Idl, out: &str) -> Result<()> {
    // todo: generate a template string and write to the output file
    let tera = Tera::new("templates/dotnet/*").unwrap();

    let mut context = Context::new();
    context.insert("name", &idl.name.to_camel_case());
    context.insert("accounts", &generate_types(&idl.accounts, &tera));
    context.insert("types", &generate_types(&idl.types, &tera));

    let res = tera.render("file.cs.tmpl", &context)?;


    println!("please be patient, currently learning rust and generating C# from IDL\n\n");

    println!("{}", &res);

    Ok(())
}

fn generate_types(types: &Vec<IdlTypeDefinition>, templates: &Tera) -> Vec<String> {
    let mut accounts = vec![];

    for ty  in types {
        let mut context = Context::new();
        context.insert("name", &ty.name.to_camel_case());

        match &ty.ty {
            IdlTypeDefinitionTy::Struct { fields } => {
                context.insert("fields", &generate_class_fields(&fields, templates));
                context.insert("methods", "");
                accounts.push(templates.render("class.cs.tmpl", &context).unwrap());
            },
            // IdlTypeDefinitionTy::Enum { variants } => {
            //     context.insert("values", &generateEnumValues(&variants));
            // }
            _ => continue
        }

    }
    accounts
}

fn generate_class_fields(idl_fields :&Vec<IdlField>, templates: &Tera) -> Vec<String> {
    let mut fields = vec![];
    for field in idl_fields {
        let mut context = Context::new();
        context.insert("type", &get_type_identifier(&field.ty));
        context.insert("name", &field.name.to_camel_case());

        fields.push(templates.render("property.cs.tmpl", &context).unwrap());
    }
    fields
}

fn get_type_identifier(typ: &IdlType) -> String {
    match typ {
        IdlType::U8 => "byte".to_owned(),
        IdlType::I8 => "sbyte".to_owned(),
        IdlType::U16 => "ushort".to_owned(),
        IdlType::I16 => "short".to_owned(),
        IdlType::U32 => "uint".to_owned(),
        IdlType::I32 => "int".to_owned(),
        IdlType::U64 => "ulong".to_owned(),
        IdlType::I64 => "long".to_owned(),
        IdlType::I128 | IdlType::U128 => "BigInteger".to_owned(),
        IdlType::PublicKey => "PublicKey".to_owned(),
        IdlType::Bool => "bool".to_owned(),
        IdlType::String  => "string".to_owned(),
        IdlType::Defined(s) => s.clone(),
        IdlType::Vec(t) | IdlType::Array(t, _) => format!("{}[]", get_type_identifier(t)),
        IdlType::Bytes => "byte[]".to_owned(),
        IdlType::Option(t) => format!("{}?", get_type_identifier(t)),
    }
}

fn generateEnumValues(values :&Vec<IdlEnumVariant>) -> Vec<String> {
    let mut fields = vec![];
    for value in values {
        fields.push(value.name.clone());
    }
    fields
}

fn is_pure_enum(values :&Vec<IdlEnumVariant>) -> bool {
    for value in values {
        match value.fields {
            Option::Some(_) => return false,
            _ => continue
        }
    }
    true
}