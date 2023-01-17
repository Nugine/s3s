use crate::f;
use crate::gen::Codegen;

use serde_json::Value;

#[derive(Debug)]
pub enum Type {
    Alias(Alias),
    Provided(Provided),
    List(List),
    Map(Map),
    UnitEnum(UnitEnum),
    Struct(Struct),
    StructEnum(StructEnum),
    Timestamp(Timestamp),
}

#[derive(Debug)]
pub struct Alias {
    pub name: String,
    pub type_: String,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct Provided {
    pub name: String,
}

#[derive(Debug)]
pub struct List {
    pub name: String,
    pub member: ListMember,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct ListMember {
    pub type_: String,
    pub xml_name: Option<String>,
}

#[derive(Debug)]
pub struct Map {
    pub name: String,
    pub key_type: String,
    pub value_type: String,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct UnitEnum {
    pub name: String,
    pub variants: Vec<UnitEnumVariant>,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct UnitEnumVariant {
    pub name: String,
    pub value: String,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub doc: Option<String>,

    pub xml_name: Option<String>,
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub type_: String,
    pub doc: Option<String>,

    pub camel_name: String,

    pub option_type: bool,
    pub default_value: Option<Value>,

    pub position: String,

    pub http_header: Option<String>,
    pub http_query: Option<String>,

    pub xml_name: Option<String>,
    pub xml_flattened: bool,
}

#[derive(Debug)]
pub struct StructEnum {
    pub name: String,
    pub variants: Vec<StructEnumVariant>,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct StructEnumVariant {
    pub name: String,
    pub type_: String,
    pub doc: Option<String>,
}

#[derive(Debug)]
pub struct Timestamp {
    pub name: String,
    pub format: Option<String>,
    pub doc: Option<String>,
}

impl Type {
    pub fn provided(name: &str) -> Self {
        Self::Provided(Provided { name: name.to_owned() })
    }

    pub fn alias(name: &str, type_: &str, doc: Option<&str>) -> Self {
        Self::Alias(Alias {
            name: name.to_owned(),
            type_: type_.to_owned(),
            doc: doc.map(ToOwned::to_owned),
        })
    }
}

pub fn codegen_doc(doc: Option<&str>, g: &mut Codegen) {
    let Some(doc) = doc else { return };

    for line in doc.lines() {
        let mut line = line.trim_start().to_owned();

        let word_fixes_type1 = [
            "Region",
            "account-id",
            "access-point-name",
            "outpost-id",
            "key",
            "version-id",
            "Code",
            "Message",
        ];

        for word in word_fixes_type1 {
            if line.contains(word) {
                line = line.replace(&f!("<{word}>"), &f!("&lt;{word}&gt;"));
            }
        }

        let word_fixes_type2 = ["Grantee", "BucketLoggingStatus"];

        for word in word_fixes_type2 {
            if line.contains(word) {
                line = line.replace(&f!("<{word} xmlns"), &f!("&lt;{word} xmlns"));
            }
        }

        let word_fixes_type3 = ["NotificationConfiguration", "Grantee"];

        for word in word_fixes_type3 {
            if line.contains(word) {
                line = line.replace(&f!("<{word}></code>"), &f!("&lt;{word}&gt;</code>"));
                line = line.replace(&f!("</{word}></code>"), &f!("&lt;/{word}&gt;</code>"));
            }
        }

        if line.ends_with("<code>200") {
            line = line.replace("<code>200", "200");
        }
        if line.starts_with("OK</code>") {
            line = line.replace("OK</code>", "OK");
        }

        g.ln(f!("/// {}", line));
    }
}
