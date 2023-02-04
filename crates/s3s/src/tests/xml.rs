// TODO: auto-generated tests

use crate::xml;

use const_str::concat;

const DATA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/tests/xml");

fn read(path: &str) -> Vec<u8> {
    std::fs::read(path).unwrap()
}

fn deserialize<T>(input: &[u8]) -> xml::DeResult<T>
where
    T: for<'xml> xml::Deserialize<'xml>,
{
    let mut d = xml::Deserializer::new(input);
    let ans = T::deserialize(&mut d)?;
    d.expect_eof()?;
    Ok(ans)
}

/// See <https://github.com/Nugine/s3s/issues/2>
#[test]
fn d001() {
    let input = read(concat!(DATA_DIR, "/d001.xml"));
    let ans = deserialize::<crate::dto::CompletedMultipartUpload>(&input).unwrap();

    let parts = ans.parts.as_deref().unwrap();
    assert_eq!(parts.len(), 3);

    assert_eq!(parts[0].part_number, 1);
    assert_eq!(parts[0].e_tag.as_deref(), Some("\"a54357aff0632cce46d942af68356b38\""));

    assert_eq!(parts[1].part_number, 2);
    assert_eq!(parts[1].e_tag.as_deref(), Some("\"0c78aef83f66abc1fa1e8477f296d394\""));

    assert_eq!(parts[2].part_number, 3);
    assert_eq!(parts[2].e_tag.as_deref(), Some("\"acbd18db4cc2f85cedef654fccc4a4d8\""));
}

#[test]
fn d002() {
    let input = read(concat!(DATA_DIR, "/d002.xml"));
    let ans = deserialize::<crate::dto::SelectObjectContentRequest>(&input).unwrap();

    {
        let csv = ans.input_serialization.csv.as_ref().unwrap();
        assert_eq!(csv.allow_quoted_record_delimiter, false);
    }
}

#[test]
fn d003() {
    let input = read(concat!(DATA_DIR, "/d003.xml"));
    let ans = deserialize::<crate::dto::Tagging>(&input).unwrap();

    assert_eq!(ans.tag_set.len(), 1);
    let tag = &ans.tag_set[0];
    assert_eq!(tag.key, "Key4");
    assert_eq!(tag.value, "Value4");
}

#[test]
fn d004() {
    let input = read(concat!(DATA_DIR, "/d004.xml"));
    let ans = deserialize::<crate::dto::SelectObjectContentRequest>(&input).unwrap();

    assert!(ans.input_serialization.csv.is_some());
    assert!(ans.output_serialization.csv.is_some());
}
