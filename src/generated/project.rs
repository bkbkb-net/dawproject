// @generated from DAWproject ee4dcdde75940f30e14e55401a26955a58b8322b with xsd-parser bf1323fca2d7c95de4c79fbae65a3183ae0d158a.
// Do not edit manually; run `cargo run --manifest-path tools/codegen/Cargo.toml --locked`.

use serde::{Deserialize, Serialize};
pub type Arrangement = ArrangementType;
pub type AuPlugin = AuPluginType;
pub type Audio = AudioType;
pub type BoolParameter = BoolParameterType;
pub type BoolPoint = BoolPointType;
pub type BuiltinDevice = BuiltinDeviceType;
pub type Channel = ChannelType;
pub type ClapPlugin = AuPluginType;
pub type Clip = ClipType;
pub type ClipSlot = ClipSlotType;
pub type Clips = ClipsType;
pub type Compressor = CompressorType;
pub type Device = DeviceType;
pub type EnumParameter = EnumParameterType;
pub type EnumPoint = EnumPointType;
pub type Equalizer = EqualizerType;
pub type IntegerParameter = IntegerParameterType;
pub type IntegerPoint = EnumPointType;
pub type Lanes = LanesType;
pub type Limiter = LimiterType;
pub type Marker = MarkerType;
pub type NoiseGate = NoiseGateType;
pub type Note = NoteType;
pub type Notes = NotesType;
pub type Point = PointType;
pub type Points = PointsType;
pub type Project = ProjectType;
pub type RealParameter = RealParameterType;
pub type RealPoint = RealPointType;
pub type Scene = SceneType;
pub type TimeSignatureParameter = TimeSignatureParameterType;
pub type TimeSignaturePoint = TimeSignaturePointType;
pub type Timeline = TimelineType;
pub type Track = TrackType;
pub type Video = AudioType;
pub type Vst2Plugin = AuPluginType;
pub type Vst3Plugin = AuPluginType;
pub type Warp = WarpType;
pub type Warps = WarpsType;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ApplicationType {
    #[serde(rename = "@name")]
    pub name: ::std::string::String,
    #[serde(rename = "@version")]
    pub version: ::std::string::String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ArrangementType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Lanes")]
    pub lanes: ::core::option::Option<LanesType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Markers")]
    pub markers: ::core::option::Option<MarkersType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "TempoAutomation"
    )]
    pub tempo_automation: ::core::option::Option<PointsType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "TimeSignatureAutomation"
    )]
    pub time_signature_automation: ::core::option::Option<PointsType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuPluginType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@pluginVersion"
    )]
    pub plugin_version: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AudioType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@duration")]
    pub duration: ::core::primitive::f64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@algorithm"
    )]
    pub algorithm: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@channels")]
    pub channels: ::core::primitive::i32,
    #[serde(rename = "@sampleRate")]
    pub sample_rate: ::core::primitive::i32,
    #[serde(rename = "File")]
    pub file: FileReferenceType,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AutomationTargetType {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@parameter"
    )]
    pub parameter: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@expression"
    )]
    pub expression: ::core::option::Option<ExpressionType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@channel")]
    pub channel: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@key")]
    pub key: ::core::option::Option<::core::primitive::i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@controller"
    )]
    pub controller: ::core::option::Option<::core::primitive::i32>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BoolParameterType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@parameterID"
    )]
    pub parameter_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@value")]
    pub value: ::core::option::Option<::core::primitive::bool>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BoolPointType {
    #[serde(rename = "@time")]
    pub time: ::std::string::String,
    #[serde(rename = "@value")]
    pub value: ::core::primitive::bool,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BuiltinDeviceType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChannelType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@audioChannels"
    )]
    pub audio_channels: ::core::option::Option<::core::primitive::i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@destination"
    )]
    pub destination: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@role")]
    pub role: ::core::option::Option<MixerRoleType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@solo")]
    pub solo: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Devices")]
    pub devices: ::core::option::Option<ChannelDevicesElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Mute")]
    pub mute: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Pan")]
    pub pan: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Sends")]
    pub sends: ::core::option::Option<ChannelSendsElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Volume")]
    pub volume: ::core::option::Option<RealParameterType>,
}
pub type ClapPluginType = AuPluginType;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClipType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@time")]
    pub time: ::core::primitive::f64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@duration")]
    pub duration: ::core::option::Option<::core::primitive::f64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@contentTimeUnit"
    )]
    pub content_time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@playStart"
    )]
    pub play_start: ::core::option::Option<::core::primitive::f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@playStop")]
    pub play_stop: ::core::option::Option<::core::primitive::f64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@loopStart"
    )]
    pub loop_start: ::core::option::Option<::core::primitive::f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loopEnd")]
    pub loop_end: ::core::option::Option<::core::primitive::f64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@fadeTimeUnit"
    )]
    pub fade_time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@fadeInTime"
    )]
    pub fade_in_time: ::core::option::Option<::core::primitive::f64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@fadeOutTime"
    )]
    pub fade_out_time: ::core::option::Option<::core::primitive::f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@enable")]
    pub enable: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@reference"
    )]
    pub reference: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "$value")]
    pub content: ::core::option::Option<ClipTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ClipTypeContent {
    #[serde(rename = "Timeline")]
    Timeline(TimelineType),
    #[serde(rename = "Lanes")]
    Lanes(LanesType),
    #[serde(rename = "Notes")]
    Notes(NotesType),
    #[serde(rename = "Clips")]
    Clips(ClipsType),
    #[serde(rename = "ClipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "Warps")]
    Warps(WarpsType),
    #[serde(rename = "Audio")]
    Audio(AudioType),
    #[serde(rename = "Video")]
    Video(AudioType),
    #[serde(rename = "Points")]
    Points(PointsType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClipSlotType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@hasStop")]
    pub has_stop: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Clip")]
    pub clip: ::core::option::Option<::std::boxed::Box<ClipType>>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ClipsType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "Clip")]
    pub clip: ::std::vec::Vec<ClipType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CompressorType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Attack")]
    pub attack: ::core::option::Option<RealParameterType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "AutoMakeup"
    )]
    pub auto_makeup: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "InputGain")]
    pub input_gain: ::core::option::Option<RealParameterType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "OutputGain"
    )]
    pub output_gain: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Ratio")]
    pub ratio: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Release")]
    pub release: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Threshold")]
    pub threshold: ::core::option::Option<RealParameterType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ContentType {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "automation")]
    Automation,
    #[serde(rename = "notes")]
    Notes,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "markers")]
    Markers,
    #[serde(rename = "tracks")]
    Tracks,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DeviceType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DeviceRoleType {
    #[serde(rename = "instrument")]
    Instrument,
    #[serde(rename = "noteFX")]
    NoteFx,
    #[serde(rename = "audioFX")]
    AudioFx,
    #[serde(rename = "analyzer")]
    Analyzer,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumParameterType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@parameterID"
    )]
    pub parameter_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(rename = "@count")]
    pub count: ::core::primitive::i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@labels")]
    pub labels: ::core::option::Option<StringList>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@value")]
    pub value: ::core::option::Option<::core::primitive::i32>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnumPointType {
    #[serde(rename = "@time")]
    pub time: ::std::string::String,
    #[serde(rename = "@value")]
    pub value: ::core::primitive::i32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EqBandType {
    #[serde(rename = "@type")]
    pub type_: EqBandKindType,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@order")]
    pub order: ::core::option::Option<::core::primitive::i32>,
    #[serde(rename = "Freq")]
    pub freq: RealParameterType,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Gain")]
    pub gain: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Q")]
    pub q: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum EqBandKindType {
    #[serde(rename = "highPass")]
    HighPass,
    #[serde(rename = "lowPass")]
    LowPass,
    #[serde(rename = "bandPass")]
    BandPass,
    #[serde(rename = "highShelf")]
    HighShelf,
    #[serde(rename = "lowShelf")]
    LowShelf,
    #[serde(rename = "bell")]
    Bell,
    #[serde(rename = "notch")]
    Notch,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EqualizerType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "Band")]
    pub band: ::std::vec::Vec<EqBandType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "InputGain")]
    pub input_gain: ::core::option::Option<RealParameterType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "OutputGain"
    )]
    pub output_gain: ::core::option::Option<RealParameterType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ExpressionType {
    #[serde(rename = "gain")]
    Gain,
    #[serde(rename = "pan")]
    Pan,
    #[serde(rename = "transpose")]
    Transpose,
    #[serde(rename = "timbre")]
    Timbre,
    #[serde(rename = "formant")]
    Formant,
    #[serde(rename = "pressure")]
    Pressure,
    #[serde(rename = "channelController")]
    ChannelController,
    #[serde(rename = "channelPressure")]
    ChannelPressure,
    #[serde(rename = "polyPressure")]
    PolyPressure,
    #[serde(rename = "pitchBend")]
    PitchBend,
    #[serde(rename = "programChange")]
    ProgramChange,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FileReferenceType {
    #[serde(rename = "@path")]
    pub path: ::std::string::String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@external")]
    pub external: ::core::option::Option<::core::primitive::bool>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct IntegerParameterType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@parameterID"
    )]
    pub parameter_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@max")]
    pub max: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@min")]
    pub min: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@value")]
    pub value: ::core::option::Option<::core::primitive::i32>,
}
pub type IntegerPointType = EnumPointType;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum InterpolationType {
    #[serde(rename = "hold")]
    Hold,
    #[serde(rename = "linear")]
    Linear,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum LaneType {
    #[serde(rename = "track")]
    Track(TrackType),
    #[serde(rename = "channel")]
    Channel(ChannelType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LaneDyn {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LanesType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "$value")]
    pub content: ::std::vec::Vec<LanesTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum LanesTypeContent {
    #[serde(rename = "Timeline")]
    Timeline(TimelineType),
    #[serde(rename = "Lanes")]
    Lanes(LanesType),
    #[serde(rename = "Notes")]
    Notes(NotesType),
    #[serde(rename = "Clips")]
    Clips(ClipsType),
    #[serde(rename = "ClipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "Warps")]
    Warps(WarpsType),
    #[serde(rename = "Audio")]
    Audio(AudioType),
    #[serde(rename = "Video")]
    Video(AudioType),
    #[serde(rename = "Points")]
    Points(PointsType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LimiterType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Attack")]
    pub attack: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "InputGain")]
    pub input_gain: ::core::option::Option<RealParameterType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "OutputGain"
    )]
    pub output_gain: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Release")]
    pub release: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Threshold")]
    pub threshold: ::core::option::Option<RealParameterType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MarkerType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@time")]
    pub time: ::core::primitive::f64,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MarkersType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "Marker")]
    pub marker: ::std::vec::Vec<MarkerType>,
}
pub type Markers = MarkersType;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MediaFileType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@duration")]
    pub duration: ::core::primitive::f64,
    #[serde(rename = "File")]
    pub file: FileReferenceType,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MixerRoleType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "effect")]
    Effect,
    #[serde(rename = "submix")]
    Submix,
    #[serde(rename = "vca")]
    Vca,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NameableType {
    #[serde(rename = "referenceable")]
    Referenceable(ReferenceableDyn),
    #[serde(rename = "parameter")]
    Parameter(ParameterDyn),
    #[serde(rename = "realParameter")]
    RealParameter(RealParameterType),
    #[serde(rename = "boolParameter")]
    BoolParameter(BoolParameterType),
    #[serde(rename = "integerParameter")]
    IntegerParameter(IntegerParameterType),
    #[serde(rename = "enumParameter")]
    EnumParameter(EnumParameterType),
    #[serde(rename = "timeSignatureParameter")]
    TimeSignatureParameter(TimeSignatureParameterType),
    #[serde(rename = "lane")]
    Lane(LaneDyn),
    #[serde(rename = "arrangement")]
    Arrangement(ArrangementType),
    #[serde(rename = "timeline")]
    Timeline(TimelineDyn),
    #[serde(rename = "lanes")]
    Lanes(LanesType),
    #[serde(rename = "track")]
    Track(TrackType),
    #[serde(rename = "channel")]
    Channel(ChannelType),
    #[serde(rename = "device")]
    Device(DeviceType),
    #[serde(rename = "plugin")]
    Plugin(PluginDyn),
    #[serde(rename = "vst2Plugin")]
    Vst2Plugin(AuPluginType),
    #[serde(rename = "vst3Plugin")]
    Vst3Plugin(AuPluginType),
    #[serde(rename = "clapPlugin")]
    ClapPlugin(AuPluginType),
    #[serde(rename = "builtinDevice")]
    BuiltinDevice(BuiltinDeviceType),
    #[serde(rename = "equalizer")]
    Equalizer(EqualizerType),
    #[serde(rename = "compressor")]
    Compressor(CompressorType),
    #[serde(rename = "noiseGate")]
    NoiseGate(NoiseGateType),
    #[serde(rename = "limiter")]
    Limiter(LimiterType),
    #[serde(rename = "auPlugin")]
    AuPlugin(AuPluginType),
    #[serde(rename = "send")]
    Send(SendType),
    #[serde(rename = "notes")]
    Notes(NotesType),
    #[serde(rename = "clip")]
    Clip(ClipType),
    #[serde(rename = "clips")]
    Clips(ClipsType),
    #[serde(rename = "clipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "marker")]
    Marker(MarkerType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "warps")]
    Warps(WarpsType),
    #[serde(rename = "mediaFile")]
    MediaFile(MediaFileType),
    #[serde(rename = "audio")]
    Audio(AudioType),
    #[serde(rename = "video")]
    Video(AudioType),
    #[serde(rename = "points")]
    Points(PointsType),
    #[serde(rename = "scene")]
    Scene(SceneType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NameableDyn {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NoiseGateType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Attack")]
    pub attack: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Range")]
    pub range: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Ratio")]
    pub ratio: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Release")]
    pub release: ::core::option::Option<RealParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Threshold")]
    pub threshold: ::core::option::Option<RealParameterType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NoteType {
    #[serde(rename = "@time")]
    pub time: ::std::string::String,
    #[serde(rename = "@duration")]
    pub duration: ::std::string::String,
    #[serde(rename = "@channel")]
    pub channel: ::core::primitive::i32,
    #[serde(rename = "@key")]
    pub key: ::core::primitive::i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@vel")]
    pub vel: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@rel")]
    pub rel: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "$value")]
    pub content: ::core::option::Option<NoteTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NoteTypeContent {
    #[serde(rename = "Timeline")]
    Timeline(TimelineType),
    #[serde(rename = "Lanes")]
    Lanes(LanesType),
    #[serde(rename = "Notes")]
    Notes(NotesType),
    #[serde(rename = "Clips")]
    Clips(ClipsType),
    #[serde(rename = "ClipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "Warps")]
    Warps(WarpsType),
    #[serde(rename = "Audio")]
    Audio(AudioType),
    #[serde(rename = "Video")]
    Video(AudioType),
    #[serde(rename = "Points")]
    Points(PointsType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NotesType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "Note")]
    pub note: ::std::vec::Vec<NoteType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ParameterType {
    #[serde(rename = "realParameter")]
    RealParameter(RealParameterType),
    #[serde(rename = "boolParameter")]
    BoolParameter(BoolParameterType),
    #[serde(rename = "integerParameter")]
    IntegerParameter(IntegerParameterType),
    #[serde(rename = "enumParameter")]
    EnumParameter(EnumParameterType),
    #[serde(rename = "timeSignatureParameter")]
    TimeSignatureParameter(TimeSignatureParameterType),
}
pub type Parameter = ParameterType;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ParameterDyn {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@parameterID"
    )]
    pub parameter_id: ::core::option::Option<::core::primitive::i32>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PluginType {
    #[serde(rename = "vst2Plugin")]
    Vst2Plugin(AuPluginType),
    #[serde(rename = "vst3Plugin")]
    Vst3Plugin(AuPluginType),
    #[serde(rename = "clapPlugin")]
    ClapPlugin(AuPluginType),
    #[serde(rename = "auPlugin")]
    AuPlugin(AuPluginType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PluginDyn {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@deviceID")]
    pub device_id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@deviceName")]
    pub device_name: ::std::string::String,
    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRoleType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@deviceVendor"
    )]
    pub device_vendor: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@pluginVersion"
    )]
    pub plugin_version: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Parameters"
    )]
    pub parameters: ::core::option::Option<DeviceParametersElementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enabled")]
    pub enabled: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "State")]
    pub state: ::core::option::Option<FileReferenceType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PointType {
    #[serde(rename = "realPoint")]
    RealPoint(RealPointType),
    #[serde(rename = "enumPoint")]
    EnumPoint(EnumPointType),
    #[serde(rename = "boolPoint")]
    BoolPoint(BoolPointType),
    #[serde(rename = "integerPoint")]
    IntegerPoint(EnumPointType),
    #[serde(rename = "timeSignaturePoint")]
    TimeSignaturePoint(TimeSignaturePointType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PointDyn {
    #[serde(rename = "@time")]
    pub time: ::std::string::String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PointsType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@unit")]
    pub unit: ::core::option::Option<UnitType>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<PointsTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PointsTypeContent {
    #[serde(rename = "Target")]
    Target(AutomationTargetType),
    #[serde(rename = "Point")]
    Point(PointType),
    #[serde(rename = "RealPoint")]
    RealPoint(RealPointType),
    #[serde(rename = "EnumPoint")]
    EnumPoint(EnumPointType),
    #[serde(rename = "BoolPoint")]
    BoolPoint(BoolPointType),
    #[serde(rename = "IntegerPoint")]
    IntegerPoint(EnumPointType),
    #[serde(rename = "TimeSignaturePoint")]
    TimeSignaturePoint(TimeSignaturePointType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProjectType {
    #[serde(rename = "@version")]
    pub version: ::std::string::String,
    #[serde(rename = "Application")]
    pub application: ApplicationType,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Transport")]
    pub transport: ::core::option::Option<TransportType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Structure")]
    pub structure: ::core::option::Option<ProjectStructureElementType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "Arrangement"
    )]
    pub arrangement: ::core::option::Option<ArrangementType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Scenes")]
    pub scenes: ::core::option::Option<ProjectScenesElementType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RealParameterType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@parameterID"
    )]
    pub parameter_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@max")]
    pub max: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@min")]
    pub min: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@unit")]
    pub unit: UnitType,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@value")]
    pub value: ::core::option::Option<::std::string::String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RealPointType {
    #[serde(rename = "@time")]
    pub time: ::std::string::String,
    #[serde(rename = "@value")]
    pub value: ::std::string::String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@interpolation"
    )]
    pub interpolation: ::core::option::Option<InterpolationType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ReferenceableType {
    #[serde(rename = "parameter")]
    Parameter(ParameterDyn),
    #[serde(rename = "realParameter")]
    RealParameter(RealParameterType),
    #[serde(rename = "boolParameter")]
    BoolParameter(BoolParameterType),
    #[serde(rename = "integerParameter")]
    IntegerParameter(IntegerParameterType),
    #[serde(rename = "enumParameter")]
    EnumParameter(EnumParameterType),
    #[serde(rename = "timeSignatureParameter")]
    TimeSignatureParameter(TimeSignatureParameterType),
    #[serde(rename = "lane")]
    Lane(LaneDyn),
    #[serde(rename = "arrangement")]
    Arrangement(ArrangementType),
    #[serde(rename = "timeline")]
    Timeline(TimelineDyn),
    #[serde(rename = "lanes")]
    Lanes(LanesType),
    #[serde(rename = "track")]
    Track(TrackType),
    #[serde(rename = "channel")]
    Channel(ChannelType),
    #[serde(rename = "device")]
    Device(DeviceType),
    #[serde(rename = "plugin")]
    Plugin(PluginDyn),
    #[serde(rename = "vst2Plugin")]
    Vst2Plugin(AuPluginType),
    #[serde(rename = "vst3Plugin")]
    Vst3Plugin(AuPluginType),
    #[serde(rename = "clapPlugin")]
    ClapPlugin(AuPluginType),
    #[serde(rename = "builtinDevice")]
    BuiltinDevice(BuiltinDeviceType),
    #[serde(rename = "equalizer")]
    Equalizer(EqualizerType),
    #[serde(rename = "compressor")]
    Compressor(CompressorType),
    #[serde(rename = "noiseGate")]
    NoiseGate(NoiseGateType),
    #[serde(rename = "limiter")]
    Limiter(LimiterType),
    #[serde(rename = "auPlugin")]
    AuPlugin(AuPluginType),
    #[serde(rename = "send")]
    Send(SendType),
    #[serde(rename = "notes")]
    Notes(NotesType),
    #[serde(rename = "clips")]
    Clips(ClipsType),
    #[serde(rename = "clipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "warps")]
    Warps(WarpsType),
    #[serde(rename = "mediaFile")]
    MediaFile(MediaFileType),
    #[serde(rename = "audio")]
    Audio(AudioType),
    #[serde(rename = "video")]
    Video(AudioType),
    #[serde(rename = "points")]
    Points(PointsType),
    #[serde(rename = "scene")]
    Scene(SceneType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ReferenceableDyn {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SceneType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: SceneTypeContent,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SceneTypeContent {
    #[serde(rename = "Timeline")]
    Timeline(TimelineType),
    #[serde(rename = "Lanes")]
    Lanes(LanesType),
    #[serde(rename = "Notes")]
    Notes(NotesType),
    #[serde(rename = "Clips")]
    Clips(ClipsType),
    #[serde(rename = "ClipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "Warps")]
    Warps(WarpsType),
    #[serde(rename = "Audio")]
    Audio(AudioType),
    #[serde(rename = "Video")]
    Video(AudioType),
    #[serde(rename = "Points")]
    Points(PointsType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SendType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@destination"
    )]
    pub destination: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@type")]
    pub type_: ::core::option::Option<SendKindType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Enable")]
    pub enable: ::core::option::Option<BoolParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Pan")]
    pub pan: ::core::option::Option<RealParameterType>,
    #[serde(rename = "Volume")]
    pub volume: RealParameterType,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SendKindType {
    #[serde(rename = "pre")]
    Pre,
    #[serde(rename = "post")]
    Post,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TimeSignatureParameterType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@parameterID"
    )]
    pub parameter_id: ::core::option::Option<::core::primitive::i32>,
    #[serde(rename = "@denominator")]
    pub denominator: ::core::primitive::i32,
    #[serde(rename = "@numerator")]
    pub numerator: ::core::primitive::i32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TimeSignaturePointType {
    #[serde(rename = "@time")]
    pub time: ::std::string::String,
    #[serde(rename = "@numerator")]
    pub numerator: ::core::primitive::i32,
    #[serde(rename = "@denominator")]
    pub denominator: ::core::primitive::i32,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TimeUnitType {
    #[serde(rename = "beats")]
    Beats,
    #[serde(rename = "seconds")]
    Seconds,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TimelineType {
    #[serde(rename = "lanes")]
    Lanes(::std::boxed::Box<LanesType>),
    #[serde(rename = "notes")]
    Notes(::std::boxed::Box<NotesType>),
    #[serde(rename = "clips")]
    Clips(::std::boxed::Box<ClipsType>),
    #[serde(rename = "clipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "warps")]
    Warps(WarpsType),
    #[serde(rename = "mediaFile")]
    MediaFile(MediaFileType),
    #[serde(rename = "audio")]
    Audio(AudioType),
    #[serde(rename = "video")]
    Video(AudioType),
    #[serde(rename = "points")]
    Points(PointsType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TimelineDyn {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TrackType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "@contentTypes"
    )]
    pub content_types: ::core::option::Option<ContentTypeList>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@loaded")]
    pub loaded: ::core::option::Option<::core::primitive::bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Channel")]
    pub channel: ::core::option::Option<ChannelType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "Track")]
    pub track: ::std::vec::Vec<TrackType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TransportType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "Tempo")]
    pub tempo: ::core::option::Option<RealParameterType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "TimeSignature"
    )]
    pub time_signature: ::core::option::Option<TimeSignatureParameterType>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum UnitType {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "normalized")]
    Normalized,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "decibel")]
    Decibel,
    #[serde(rename = "hertz")]
    Hertz,
    #[serde(rename = "semitones")]
    Semitones,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "beats")]
    Beats,
    #[serde(rename = "bpm")]
    Bpm,
}
pub type VideoType = AudioType;
pub type Vst2PluginType = AuPluginType;
pub type Vst3PluginType = AuPluginType;
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WarpType {
    #[serde(rename = "@time")]
    pub time: ::core::primitive::f64,
    #[serde(rename = "@contentTime")]
    pub content_time: ::core::primitive::f64,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WarpsType {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@name")]
    pub name: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@color")]
    pub color: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@comment")]
    pub comment: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@timeUnit")]
    pub time_unit: ::core::option::Option<TimeUnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "@track")]
    pub track: ::core::option::Option<::std::string::String>,
    #[serde(rename = "@contentTimeUnit")]
    pub content_time_unit: TimeUnitType,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<WarpsTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum WarpsTypeContent {
    #[serde(rename = "Timeline")]
    Timeline(TimelineType),
    #[serde(rename = "Lanes")]
    Lanes(LanesType),
    #[serde(rename = "Notes")]
    Notes(NotesType),
    #[serde(rename = "Clips")]
    Clips(ClipsType),
    #[serde(rename = "ClipSlot")]
    ClipSlot(ClipSlotType),
    #[serde(rename = "markers")]
    Markers(MarkersType),
    #[serde(rename = "Warps")]
    Warps(WarpsType),
    #[serde(rename = "Audio")]
    Audio(AudioType),
    #[serde(rename = "Video")]
    Video(AudioType),
    #[serde(rename = "Points")]
    Points(PointsType),
    #[serde(rename = "Warp")]
    Warp(WarpType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DeviceParametersElementType {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "$value")]
    pub content: ::std::vec::Vec<DeviceParametersElementTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DeviceParametersElementTypeContent {
    #[serde(rename = "parameter")]
    Parameter(ParameterType),
    #[serde(rename = "RealParameter")]
    RealParameter(RealParameterType),
    #[serde(rename = "BoolParameter")]
    BoolParameter(BoolParameterType),
    #[serde(rename = "IntegerParameter")]
    IntegerParameter(IntegerParameterType),
    #[serde(rename = "EnumParameter")]
    EnumParameter(EnumParameterType),
    #[serde(rename = "TimeSignatureParameter")]
    TimeSignatureParameter(TimeSignatureParameterType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChannelDevicesElementType {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "$value")]
    pub content: ::std::vec::Vec<ChannelDevicesElementTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ChannelDevicesElementTypeContent {
    #[serde(rename = "Device")]
    Device(DeviceType),
    #[serde(rename = "Vst2Plugin")]
    Vst2Plugin(AuPluginType),
    #[serde(rename = "Vst3Plugin")]
    Vst3Plugin(AuPluginType),
    #[serde(rename = "ClapPlugin")]
    ClapPlugin(AuPluginType),
    #[serde(rename = "BuiltinDevice")]
    BuiltinDevice(BuiltinDeviceType),
    #[serde(rename = "Equalizer")]
    Equalizer(EqualizerType),
    #[serde(rename = "Compressor")]
    Compressor(CompressorType),
    #[serde(rename = "NoiseGate")]
    NoiseGate(NoiseGateType),
    #[serde(rename = "Limiter")]
    Limiter(LimiterType),
    #[serde(rename = "AuPlugin")]
    AuPlugin(AuPluginType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChannelSendsElementType {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "Send")]
    pub send: ::std::vec::Vec<SendType>,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StringList(pub ::std::vec::Vec<xs::StringType>);
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProjectStructureElementType {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "$value")]
    pub content: ::std::vec::Vec<ProjectStructureElementTypeContent>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ProjectStructureElementTypeContent {
    #[serde(rename = "Track")]
    Track(TrackType),
    #[serde(rename = "Channel")]
    Channel(ChannelType),
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProjectScenesElementType {
    #[serde(default, skip_serializing_if = "Vec::is_empty", rename = "Scene")]
    pub scene: ::std::vec::Vec<SceneType>,
}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContentTypeList(pub ::std::vec::Vec<ContentType>);
pub mod xs {
    use serde::{Deserialize, Serialize};
    pub type EntitiesType = super::StringList;
    pub type EntityType = ::std::string::String;
    pub type IdType = ::std::string::String;
    pub type IdrefType = ::std::string::String;
    pub type IdrefsType = super::StringList;
    pub type NcNameType = ::std::string::String;
    pub type NmtokenType = ::std::string::String;
    pub type NmtokensType = super::StringList;
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
