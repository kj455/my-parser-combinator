use crate::json::parse_json;

mod cat;
mod char;
mod digits;
mod helper;
mod json;
mod lexeme;
mod map;
mod or;
mod regexp;
mod rep;
mod split;
mod string;

fn main() {
    const json_str: &str = r#"{
   "glossary":{
      "title":"example glossary",
      "GlossDiv":{
         "title":"S",
         "GlossList":{
            "GlossEntry":{
               "ID":"SGML",
               "SortAs":"SGML",
               "GlossTerm":"Standard Generalized Markup Language",
               "Acronym":"SGML",
               "Abbrev":"ISO 8879:1986",
               "GlossDef":{
                  "para":"A meta-markup language, used to create markup languages such as DocBook.",
                  "GlossSeeAlso":[
                     "GML",
                     "XML"
                  ]
               },
               "GlossSee":"markup"
            }
         }
      }
   }
}"#;
    if let Some(json) = parse_json(json_str) {
        println!("{:?}", json);
    }
}
