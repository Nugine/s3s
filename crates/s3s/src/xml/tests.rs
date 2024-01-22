use crate::xml;

fn deserialize<T>(input: &[u8]) -> xml::DeResult<T>
where
    T: for<'xml> xml::Deserialize<'xml>,
{
    let mut d = xml::Deserializer::new(input);
    let ans = T::deserialize(&mut d)?;
    d.expect_eof()?;
    Ok(ans)
}

fn serialize_content<T: xml::SerializeContent>(val: &T) -> xml::SerResult<String> {
    let mut buf = Vec::with_capacity(256);
    {
        let mut ser = xml::Serializer::new(&mut buf);
        val.serialize_content(&mut ser)?;
    }
    Ok(String::from_utf8(buf).unwrap())
}

fn serialize<T: xml::Serialize>(val: &T) -> xml::SerResult<String> {
    let mut buf = Vec::with_capacity(256);
    {
        let mut ser = xml::Serializer::new(&mut buf);
        val.serialize(&mut ser)?;
    }
    Ok(String::from_utf8(buf).unwrap())
}

/// See <https://github.com/Nugine/s3s/issues/2>
#[test]
fn d001() {
    let input = r#"
        <CompleteMultipartUpload>
            <Part>
                <PartNumber>1</PartNumber>
                <ETag>"a54357aff0632cce46d942af68356b38"</ETag>
            </Part>
            <Part>
                <PartNumber>2</PartNumber>
                <ETag>"0c78aef83f66abc1fa1e8477f296d394"</ETag>
            </Part>
            <Part>
                <PartNumber>3</PartNumber>
                <ETag>"acbd18db4cc2f85cedef654fccc4a4d8"</ETag>
            </Part>
        </CompleteMultipartUpload>
    "#;

    let ans = deserialize::<crate::dto::CompletedMultipartUpload>(input.as_bytes()).unwrap();

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
    let input = r#"
        <SelectObjectContentRequest xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <Expression>select * from s3object</Expression>
            <ExpressionType>SQL</ExpressionType>
            <InputSerialization>
                <CompressionType>NONE</CompressionType>
                <CSV>
                    <AllowQuotedRecordDelimiter>FALSE</AllowQuotedRecordDelimiter>
                    <Comments>#</Comments>
                    <FieldDelimiter>,</FieldDelimiter>
                    <FileHeaderInfo>NONE</FileHeaderInfo>
                    <QuoteCharacter>""</QuoteCharacter>
                    <QuoteEscapeCharacter>"</QuoteEscapeCharacter>
                    <RecordDelimiter>\n</RecordDelimiter>
                </CSV>
            </InputSerialization>
            <OutputSerialization>
                <JSON>
                    <RecordDelimiter>\n</RecordDelimiter>
                </JSON>
            </OutputSerialization>
        </SelectObjectContentRequest>
    "#;

    let ans = deserialize::<crate::dto::SelectObjectContentRequest>(input.as_bytes()).unwrap();

    {
        let csv = ans.input_serialization.csv.as_ref().unwrap();
        assert_eq!(csv.allow_quoted_record_delimiter, false);
    }
}

#[test]
fn d003() {
    let input = r#"
        <Tagging xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <TagSet>
                <Tag>
                    <Key>Key4</Key>
                    <Value>Value4</Value>
                </Tag>
            </TagSet>
        </Tagging>
    "#;

    let ans = deserialize::<crate::dto::Tagging>(input.as_bytes()).unwrap();

    assert_eq!(ans.tag_set.len(), 1);
    let tag = &ans.tag_set[0];
    assert_eq!(tag.key, "Key4");
    assert_eq!(tag.value, "Value4");
}

#[test]
fn d004() {
    let input = r#"
        <SelectObjectContentRequest xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <Expression>select * from s3object</Expression>
            <ExpressionType>SQL</ExpressionType>
            <InputSerialization>
                <CSV />
            </InputSerialization>
            <OutputSerialization>
                <CSV />
            </OutputSerialization>
            <RequestProgress>
                <Enabled>true</Enabled>
            </RequestProgress>
        </SelectObjectContentRequest>
    "#;

    let ans = deserialize::<crate::dto::SelectObjectContentRequest>(input.as_bytes()).unwrap();

    assert!(ans.input_serialization.csv.is_some());
    assert!(ans.output_serialization.csv.is_some());
}

#[test]
fn s001() {
    let val = crate::dto::LifecycleExpiration {
        date: None,
        days: Some(365),
        expired_object_delete_marker: None,
    };

    let ans = serialize_content(&val).unwrap();
    let expected = "<Days>365</Days>";

    assert_eq!(ans, expected);
}

#[test]
fn s002() {
    {
        let us_west_2 = crate::dto::BucketLocationConstraint::from_static(crate::dto::BucketLocationConstraint::US_WEST_2);
        let val = crate::dto::GetBucketLocationOutput {
            location_constraint: Some(us_west_2),
        };

        let ans = serialize(&val).unwrap();
        let expected = "<LocationConstraint>us-west-2</LocationConstraint>";

        assert_eq!(ans, expected);
    }
    {
        let val = crate::dto::GetBucketLocationOutput {
            location_constraint: None,
        };

        let ans = serialize(&val).unwrap();
        let expected = "<LocationConstraint></LocationConstraint>";

        assert_eq!(ans, expected);
    }
}
