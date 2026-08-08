// @generated from DAWproject ee4dcdde75940f30e14e55401a26955a58b8322b with xsd-parser bf1323fca2d7c95de4c79fbae65a3183ae0d158a.
// Do not edit manually; run `cargo run --manifest-path tools/codegen/Cargo.toml --locked`.

use serde::{Deserialize, Serialize};
pub type MetaData = MetaDataType;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MetaDataType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Title")]
    pub title: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Artist")]
    pub artist: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Album")]
    pub album: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "OriginalArtist"
    )]
    pub original_artist: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Composer")]
    pub composer: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Songwriter"
    )]
    pub songwriter: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Producer")]
    pub producer: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Arranger")]
    pub arranger: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Year")]
    pub year: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Genre")]
    pub genre: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Copyright")]
    pub copyright: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Website")]
    pub website: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Comment")]
    pub comment: ::core::option::Option<::std::string::String>,
}
pub mod xs {
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
    pub struct EntitiesType(pub ::std::vec::Vec<::std::string::String>);
    pub type EntityType = ::std::string::String;
    pub type IdType = ::std::string::String;
    pub type IdrefType = ::std::string::String;
    pub type IdrefsType = EntitiesType;
    pub type NcNameType = ::std::string::String;
    pub type NmtokenType = ::std::string::String;
    pub type NmtokensType = EntitiesType;
    pub type NotationType = ::std::string::String;
    pub type NameType = ::std::string::String;
    pub type QNameType = ::std::string::String;
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct AnySimpleType {
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "@type")]
        pub type_: ::core::option::Option<::std::string::String>,
        #[serde(default, rename = "$text")]
        pub content: ::std::string::String,
    }
    pub type AnyUriType = ::std::string::String;
    pub type Base64BinaryType = crate::xml_types::Base64String;
    pub type BooleanType = ::core::primitive::bool;
    pub type ByteType = ::core::primitive::i8;
    pub type DateType = ::std::string::String;
    pub type DateTimeType = ::std::string::String;
    pub type DecimalType = ::core::primitive::f64;
    pub type DoubleType = ::core::primitive::f64;
    pub type DurationType = ::std::string::String;
    pub type FloatType = ::core::primitive::f32;
    pub type GDayType = ::std::string::String;
    pub type GMonthType = ::std::string::String;
    pub type GMonthDayType = ::std::string::String;
    pub type GYearType = ::std::string::String;
    pub type GYearMonthType = ::std::string::String;
    pub type HexBinaryType = crate::xml_types::HexString;
    pub type IntType = ::core::primitive::i32;
    pub type IntegerType = ::num::BigInt;
    pub type LanguageType = ::std::string::String;
    pub type LongType = ::core::primitive::i64;
    pub type NegativeIntegerType = ::core::num::NonZeroIsize;
    pub type NonNegativeIntegerType = ::num::BigUint;
    pub type NonPositiveIntegerType = ::num::BigInt;
    pub type NormalizedStringType = ::std::string::String;
    pub type PositiveIntegerType = ::core::num::NonZeroUsize;
    pub type ShortType = ::core::primitive::i16;
    pub type StringType = ::std::string::String;
    pub type TimeType = ::std::string::String;
    pub type TokenType = ::std::string::String;
    pub type UnsignedByteType = ::core::primitive::u8;
    pub type UnsignedIntType = ::core::primitive::u32;
    pub type UnsignedLongType = ::core::primitive::u64;
    pub type UnsignedShortType = ::core::primitive::u16;
}
