use dawproject::prelude::{NameableTrait, ParameterTrait, PointTrait, ReferenceableTrait};
use dawproject::project::{
    BoolPointType, NameableType, ParameterType, RealParameterType, ReferenceableDyn, UnitType,
};
use dawproject::xml_types::{Base64String, HexString};
use serde::{Deserialize, Serialize};

#[test]
fn generated_enum_traits_delegate_to_inner_types() {
    let nameable = NameableType::Referenceable(ReferenceableDyn {
        name: Some("Synth".to_owned()),
        color: Some("#ff00ff".to_owned()),
        comment: Some("Lead".to_owned()),
        id: Some("device-1".to_owned()),
    });
    assert_eq!(nameable.get_name(), Some("Synth"));
    assert_eq!(nameable.get_color(), Some("#ff00ff"));
    assert_eq!(nameable.get_comment(), Some("Lead"));

    let parameter = ParameterType::RealParameter(RealParameterType {
        name: Some("Gain".to_owned()),
        color: None,
        comment: None,
        id: Some("parameter-1".to_owned()),
        parameter_id: Some(7),
        max: Some("1".to_owned()),
        min: Some("0".to_owned()),
        unit: UnitType::Linear,
        value: Some("0.5".to_owned()),
    });
    assert_eq!(parameter.get_name(), Some("Gain"));
    assert_eq!(parameter.get_id(), Some("parameter-1"));
    assert_eq!(parameter.get_parameter_id(), Some(7));

    let point = dawproject::project::PointType::BoolPoint(BoolPointType {
        time: "1.5".to_owned(),
        value: true,
    });
    assert_eq!(point.get_time(), "1.5");
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct EncodedValues {
    #[serde(rename = "@base64")]
    base64: Base64String,
    #[serde(rename = "@hex")]
    hex: HexString,
}

#[test]
fn local_xml_string_types_round_trip_as_attributes() {
    let values: EncodedValues =
        quick_xml::de::from_str(r#"<EncodedValues base64="AQI=" hex="0aff"/>"#).unwrap();

    assert_eq!(values.base64.as_str(), "AQI=");
    assert_eq!(values.base64.len(), 2);
    assert_eq!(values.hex.as_str(), "0aff");
    assert_eq!(values.hex.len(), 2);

    let xml = quick_xml::se::to_string(&values).unwrap();
    let reparsed: EncodedValues = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(reparsed, values);
}
