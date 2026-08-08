use dawproject::project::{ClipType, SendType};

#[test]
fn clip_enable_attribute_round_trips() {
    let clip: ClipType = quick_xml::de::from_str(r#"<Clip time="0" enable="true"/>"#).unwrap();

    assert_eq!(clip.enable, Some(true));

    let xml = quick_xml::se::to_string(&clip).unwrap();
    let reparsed: ClipType = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(reparsed.enable, Some(true));
}

#[test]
fn send_enable_parameter_round_trips() {
    let send: SendType = quick_xml::de::from_str(
        r#"<Send><Enable value="true"/><Volume unit="linear" value="0.5"/></Send>"#,
    )
    .unwrap();

    assert_eq!(
        send.enable.as_ref().and_then(|enable| enable.value),
        Some(true)
    );

    let xml = quick_xml::se::to_string(&send).unwrap();
    let reparsed: SendType = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(reparsed.enable.and_then(|enable| enable.value), Some(true));
}
