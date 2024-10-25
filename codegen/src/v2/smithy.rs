use std::collections::BTreeMap;

use numeric_cast::numeric_cast;
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Deserialize)]
pub struct Model {
    pub smithy: String,
    pub shapes: BTreeMap<String, Shape>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Shape {
    Boolean(BooleanShape),
    Integer(IntegerShape),
    Long(LongShape),
    String(StringShape),
    Timestamp(TimestampShape),
    Blob(BlobShape),
    List(ListShape),
    Map(MapShape),
    Enum(EnumShape),
    Union(UnionShape),
    Structure(StructureShape),
    Operation(OperationShape),
    Service(ServiceShape),
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Traits(Option<Map<String, Value>>);

#[derive(Debug, Deserialize)]
pub struct StringShape {
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct TimestampShape {
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct StructureShape {
    pub members: BTreeMap<String, StructureMember>,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct StructureMember {
    pub target: String,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct OperationShape {
    pub input: OperationInput,
    pub output: OperationOutput,
    // pub errors: Option<Vec<OperationError>>,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct OperationInput {
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct OperationOutput {
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct OperationError {
    // pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct BooleanShape {
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct ListShape {
    pub member: ListMember,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct ListMember {
    pub target: String,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct ServiceShape {
    // pub version: String,
    // pub operations: Vec<ServiceOperation>,
}

#[derive(Debug, Deserialize)]
pub struct ServiceOperation {
    // pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct UnionShape {
    pub members: BTreeMap<String, UnionMember>,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct UnionMember {
    pub target: String,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct EnumShape {
    pub members: BTreeMap<String, EnumMember>,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct EnumMember {
    // pub target: String,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct BlobShape {
    // pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct LongShape {
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct IntegerShape {
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct MapShape {
    pub key: MapKey,
    pub value: MapValue,
    pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct MapKey {
    pub target: String,
    // pub traits: Traits,
}

#[derive(Debug, Deserialize)]
pub struct MapValue {
    pub target: String,
    // pub traits: Traits,
}

impl Model {
    pub fn load_json(path: &str) -> Self {
        let json = std::fs::read_to_string(path).unwrap();
        let model: Self = serde_json::from_str(&json).unwrap();
        assert_eq!(model.smithy, "2.0");
        model
    }
}

impl Traits {
    pub fn get(&self, key: &str) -> Option<&Value> {
        let map = self.0.as_ref()?;
        map.get(key)
    }

    pub fn enum_value(&self) -> Option<&str> {
        self.get("smithy.api#enumValue")?.as_str()
    }

    pub fn doc(&self) -> Option<&str> {
        self.get("smithy.api#documentation")?.as_str()
    }

    pub fn timestamp_format(&self) -> Option<&str> {
        self.get("smithy.api#timestampFormat")?.as_str()
    }

    pub fn required(&self) -> bool {
        self.get("smithy.api#required").is_some()
    }

    pub fn default_value(&self) -> Option<&Value> {
        self.get("smithy.api#default")
    }

    pub fn http_header(&self) -> Option<&str> {
        self.get("smithy.api#httpHeader")?.as_str()
    }

    pub fn http_payload(&self) -> bool {
        self.get("smithy.api#httpPayload").is_some()
    }

    pub fn http_query(&self) -> Option<&str> {
        self.get("smithy.api#httpQuery")?.as_str()
    }

    pub fn xml_name(&self) -> Option<&str> {
        self.get("smithy.api#xmlName")?.as_str()
    }

    pub fn xml_flattened(&self) -> bool {
        self.get("smithy.api#xmlFlattened").is_some()
    }

    pub fn s3_unwrapped_xml_output(&self) -> bool {
        self.get("aws.customizations#s3UnwrappedXmlOutput").is_some()
    }

    pub fn http_label(&self) -> Option<&Value> {
        self.get("smithy.api#httpLabel")
    }

    pub fn http_prefix_headers(&self) -> Option<&str> {
        self.get("smithy.api#httpPrefixHeaders")?.as_str()
    }

    pub fn http_method(&self) -> Option<&str> {
        self.get("smithy.api#http")?.as_object()?.get("method")?.as_str()
    }

    pub fn http_uri(&self) -> Option<&str> {
        self.get("smithy.api#http")?.as_object()?.get("uri")?.as_str()
    }

    pub fn http_code(&self) -> Option<u16> {
        self.get("smithy.api#http")?
            .as_object()?
            .get("code")?
            .as_u64()
            .map(numeric_cast::<_, u16>)
    }

    pub fn error(&self) -> Option<&str> {
        self.get("smithy.api#error")?.as_str()
    }
}
