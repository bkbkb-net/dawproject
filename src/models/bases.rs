use crate::repositories::project::{
    ArrangementType, AuPluginType, AudioType, BoolParameterType, BoolPointType, BuiltinDeviceType,
    ChannelType, ClipSlotType, ClipType, ClipsType, CompressorType, DeviceParametersElementType,
    DeviceRoleType, DeviceType, EnumParameterType, EnumPointType, EqualizerType, FileReferenceType,
    IntegerParameterType, LaneDyn, LaneType, LanesType, LimiterType, MarkerType, MarkersType,
    MediaFileType, NameableDyn, NameableType, NoiseGateType, NotesType, ParameterDyn,
    ParameterType, PluginDyn, PluginType, PointDyn, PointType, PointsType, RealParameterType,
    RealPointType, ReferenceableDyn, ReferenceableType, SceneType, SendType,
    TimeSignatureParameterType, TimeSignaturePointType, TimeUnitType, TimelineDyn, TimelineType,
    TrackType, WarpsType,
};

// ---------------------------------------------------------------------------
// NameableTrait — types with name, color, comment
// ---------------------------------------------------------------------------

pub trait NameableTrait {
    fn get_name(&self) -> Option<&str>;
    fn get_color(&self) -> Option<&str>;
    fn get_comment(&self) -> Option<&str>;
}

macro_rules! impl_nameable {
    ($($type:ty),* $(,)?) => { $(
        impl NameableTrait for $type {
            fn get_name(&self) -> Option<&str> { self.name.as_deref() }
            fn get_color(&self) -> Option<&str> { self.color.as_deref() }
            fn get_comment(&self) -> Option<&str> { self.comment.as_deref() }
        }
    )* };
}

macro_rules! impl_nameable_enum {
    ($type:ty, $($variant:path),+ $(,)?) => {
        impl NameableTrait for $type {
            fn get_name(&self) -> Option<&str> {
                match self { $($variant(value) => value.get_name(),)+ }
            }
            fn get_color(&self) -> Option<&str> {
                match self { $($variant(value) => value.get_color(),)+ }
            }
            fn get_comment(&self) -> Option<&str> {
                match self { $($variant(value) => value.get_comment(),)+ }
            }
        }
    };
}

impl_nameable!(
    NameableDyn,
    ReferenceableDyn,
    ParameterDyn,
    LaneDyn,
    TimelineDyn,
    PluginDyn,
    ClipType,
    MarkerType,
    SceneType,
    ArrangementType,
    SendType,
    RealParameterType,
    IntegerParameterType,
    BoolParameterType,
    EnumParameterType,
    TimeSignatureParameterType,
    TrackType,
    ChannelType,
    LanesType,
    NotesType,
    ClipsType,
    ClipSlotType,
    MarkersType,
    WarpsType,
    MediaFileType,
    AudioType,
    PointsType,
    DeviceType,
    BuiltinDeviceType,
    EqualizerType,
    CompressorType,
    NoiseGateType,
    LimiterType,
    AuPluginType,
);

impl_nameable_enum!(
    NameableType,
    NameableType::Referenceable,
    NameableType::Parameter,
    NameableType::RealParameter,
    NameableType::BoolParameter,
    NameableType::IntegerParameter,
    NameableType::EnumParameter,
    NameableType::TimeSignatureParameter,
    NameableType::Lane,
    NameableType::Arrangement,
    NameableType::Timeline,
    NameableType::Lanes,
    NameableType::Track,
    NameableType::Channel,
    NameableType::Device,
    NameableType::Plugin,
    NameableType::Vst2Plugin,
    NameableType::Vst3Plugin,
    NameableType::ClapPlugin,
    NameableType::BuiltinDevice,
    NameableType::Equalizer,
    NameableType::Compressor,
    NameableType::NoiseGate,
    NameableType::Limiter,
    NameableType::AuPlugin,
    NameableType::Send,
    NameableType::Notes,
    NameableType::Clip,
    NameableType::Clips,
    NameableType::ClipSlot,
    NameableType::Marker,
    NameableType::Markers,
    NameableType::Warps,
    NameableType::MediaFile,
    NameableType::Audio,
    NameableType::Video,
    NameableType::Points,
    NameableType::Scene,
);

impl_nameable_enum!(
    ReferenceableType,
    ReferenceableType::Parameter,
    ReferenceableType::RealParameter,
    ReferenceableType::BoolParameter,
    ReferenceableType::IntegerParameter,
    ReferenceableType::EnumParameter,
    ReferenceableType::TimeSignatureParameter,
    ReferenceableType::Lane,
    ReferenceableType::Arrangement,
    ReferenceableType::Timeline,
    ReferenceableType::Lanes,
    ReferenceableType::Track,
    ReferenceableType::Channel,
    ReferenceableType::Device,
    ReferenceableType::Plugin,
    ReferenceableType::Vst2Plugin,
    ReferenceableType::Vst3Plugin,
    ReferenceableType::ClapPlugin,
    ReferenceableType::BuiltinDevice,
    ReferenceableType::Equalizer,
    ReferenceableType::Compressor,
    ReferenceableType::NoiseGate,
    ReferenceableType::Limiter,
    ReferenceableType::AuPlugin,
    ReferenceableType::Send,
    ReferenceableType::Notes,
    ReferenceableType::Clips,
    ReferenceableType::ClipSlot,
    ReferenceableType::Markers,
    ReferenceableType::Warps,
    ReferenceableType::MediaFile,
    ReferenceableType::Audio,
    ReferenceableType::Video,
    ReferenceableType::Points,
    ReferenceableType::Scene,
);

impl_nameable_enum!(
    ParameterType,
    ParameterType::RealParameter,
    ParameterType::BoolParameter,
    ParameterType::IntegerParameter,
    ParameterType::EnumParameter,
    ParameterType::TimeSignatureParameter,
);
impl_nameable_enum!(LaneType, LaneType::Track, LaneType::Channel);
impl_nameable_enum!(
    TimelineType,
    TimelineType::Lanes,
    TimelineType::Notes,
    TimelineType::Clips,
    TimelineType::ClipSlot,
    TimelineType::Markers,
    TimelineType::Warps,
    TimelineType::MediaFile,
    TimelineType::Audio,
    TimelineType::Video,
    TimelineType::Points,
);
impl_nameable_enum!(
    PluginType,
    PluginType::Vst2Plugin,
    PluginType::Vst3Plugin,
    PluginType::ClapPlugin,
    PluginType::AuPlugin,
);

// ---------------------------------------------------------------------------
// ReferenceableTrait — types with id
// ---------------------------------------------------------------------------

pub trait ReferenceableTrait: NameableTrait {
    fn get_id(&self) -> Option<&str>;
}

macro_rules! impl_referenceable {
    ($($type:ty),* $(,)?) => { $(
        impl ReferenceableTrait for $type {
            fn get_id(&self) -> Option<&str> { self.id.as_deref() }
        }
    )* };
}

macro_rules! impl_referenceable_enum {
    ($type:ty, $($variant:path),+ $(,)?) => {
        impl ReferenceableTrait for $type {
            fn get_id(&self) -> Option<&str> {
                match self { $($variant(value) => value.get_id(),)+ }
            }
        }
    };
}

impl_referenceable!(
    ReferenceableDyn,
    ParameterDyn,
    LaneDyn,
    TimelineDyn,
    PluginDyn,
    SceneType,
    ArrangementType,
    SendType,
    RealParameterType,
    IntegerParameterType,
    BoolParameterType,
    EnumParameterType,
    TimeSignatureParameterType,
    TrackType,
    ChannelType,
    LanesType,
    NotesType,
    ClipsType,
    ClipSlotType,
    MarkersType,
    WarpsType,
    MediaFileType,
    AudioType,
    PointsType,
    DeviceType,
    BuiltinDeviceType,
    EqualizerType,
    CompressorType,
    NoiseGateType,
    LimiterType,
    AuPluginType,
);

impl_referenceable_enum!(
    ReferenceableType,
    ReferenceableType::Parameter,
    ReferenceableType::RealParameter,
    ReferenceableType::BoolParameter,
    ReferenceableType::IntegerParameter,
    ReferenceableType::EnumParameter,
    ReferenceableType::TimeSignatureParameter,
    ReferenceableType::Lane,
    ReferenceableType::Arrangement,
    ReferenceableType::Timeline,
    ReferenceableType::Lanes,
    ReferenceableType::Track,
    ReferenceableType::Channel,
    ReferenceableType::Device,
    ReferenceableType::Plugin,
    ReferenceableType::Vst2Plugin,
    ReferenceableType::Vst3Plugin,
    ReferenceableType::ClapPlugin,
    ReferenceableType::BuiltinDevice,
    ReferenceableType::Equalizer,
    ReferenceableType::Compressor,
    ReferenceableType::NoiseGate,
    ReferenceableType::Limiter,
    ReferenceableType::AuPlugin,
    ReferenceableType::Send,
    ReferenceableType::Notes,
    ReferenceableType::Clips,
    ReferenceableType::ClipSlot,
    ReferenceableType::Markers,
    ReferenceableType::Warps,
    ReferenceableType::MediaFile,
    ReferenceableType::Audio,
    ReferenceableType::Video,
    ReferenceableType::Points,
    ReferenceableType::Scene,
);
impl_referenceable_enum!(
    ParameterType,
    ParameterType::RealParameter,
    ParameterType::BoolParameter,
    ParameterType::IntegerParameter,
    ParameterType::EnumParameter,
    ParameterType::TimeSignatureParameter,
);
impl_referenceable_enum!(LaneType, LaneType::Track, LaneType::Channel);
impl_referenceable_enum!(
    TimelineType,
    TimelineType::Lanes,
    TimelineType::Notes,
    TimelineType::Clips,
    TimelineType::ClipSlot,
    TimelineType::Markers,
    TimelineType::Warps,
    TimelineType::MediaFile,
    TimelineType::Audio,
    TimelineType::Video,
    TimelineType::Points,
);
impl_referenceable_enum!(
    PluginType,
    PluginType::Vst2Plugin,
    PluginType::Vst3Plugin,
    PluginType::ClapPlugin,
    PluginType::AuPlugin,
);

// ---------------------------------------------------------------------------
// ParameterTrait — types with parameter_id
// ---------------------------------------------------------------------------

pub trait ParameterTrait: ReferenceableTrait {
    fn get_parameter_id(&self) -> Option<i32>;
}

macro_rules! impl_parameter {
    ($($type:ty),* $(,)?) => { $(
        impl ParameterTrait for $type {
            fn get_parameter_id(&self) -> Option<i32> { self.parameter_id }
        }
    )* };
}

impl_parameter!(
    ParameterDyn,
    RealParameterType,
    IntegerParameterType,
    BoolParameterType,
    EnumParameterType,
    TimeSignatureParameterType,
);

impl ParameterTrait for ParameterType {
    fn get_parameter_id(&self) -> Option<i32> {
        match self {
            Self::RealParameter(value) => value.get_parameter_id(),
            Self::BoolParameter(value) => value.get_parameter_id(),
            Self::IntegerParameter(value) => value.get_parameter_id(),
            Self::EnumParameter(value) => value.get_parameter_id(),
            Self::TimeSignatureParameter(value) => value.get_parameter_id(),
        }
    }
}

// ---------------------------------------------------------------------------
// LaneTrait — Lane, Track, Channel
// ---------------------------------------------------------------------------

pub trait LaneTrait: ReferenceableTrait {}

impl LaneTrait for LaneDyn {}
impl LaneTrait for LaneType {}
impl LaneTrait for TrackType {}
impl LaneTrait for ChannelType {}

// ---------------------------------------------------------------------------
// TimelineTrait — types with time_unit and track reference
// ---------------------------------------------------------------------------

pub trait TimelineTrait: ReferenceableTrait {
    fn get_time_unit(&self) -> Option<&TimeUnitType>;
    fn get_track(&self) -> Option<&str>;
}

macro_rules! impl_timeline {
    ($($type:ty),* $(,)?) => { $(
        impl TimelineTrait for $type {
            fn get_time_unit(&self) -> Option<&TimeUnitType> { self.time_unit.as_ref() }
            fn get_track(&self) -> Option<&str> { self.track.as_deref() }
        }
    )* };
}

impl_timeline!(
    TimelineDyn,
    LanesType,
    NotesType,
    ClipsType,
    ClipSlotType,
    MarkersType,
    WarpsType,
    MediaFileType,
    AudioType,
    PointsType,
);

impl TimelineTrait for TimelineType {
    fn get_time_unit(&self) -> Option<&TimeUnitType> {
        match self {
            Self::Lanes(value) => value.get_time_unit(),
            Self::Notes(value) => value.get_time_unit(),
            Self::Clips(value) => value.get_time_unit(),
            Self::ClipSlot(value) => value.get_time_unit(),
            Self::Markers(value) => value.get_time_unit(),
            Self::Warps(value) => value.get_time_unit(),
            Self::MediaFile(value) => value.get_time_unit(),
            Self::Audio(value) | Self::Video(value) => value.get_time_unit(),
            Self::Points(value) => value.get_time_unit(),
        }
    }

    fn get_track(&self) -> Option<&str> {
        match self {
            Self::Lanes(value) => value.get_track(),
            Self::Notes(value) => value.get_track(),
            Self::Clips(value) => value.get_track(),
            Self::ClipSlot(value) => value.get_track(),
            Self::Markers(value) => value.get_track(),
            Self::Warps(value) => value.get_track(),
            Self::MediaFile(value) => value.get_track(),
            Self::Audio(value) | Self::Video(value) => value.get_track(),
            Self::Points(value) => value.get_track(),
        }
    }
}

// ---------------------------------------------------------------------------
// MediaFileTrait — types with file and duration
// ---------------------------------------------------------------------------

pub trait MediaFileTrait: TimelineTrait {
    fn get_file(&self) -> &FileReferenceType;
    fn get_duration(&self) -> f64;
}

macro_rules! impl_media_file {
    ($($type:ty),* $(,)?) => { $(
        impl MediaFileTrait for $type {
            fn get_file(&self) -> &FileReferenceType { &self.file }
            fn get_duration(&self) -> f64 { self.duration }
        }
    )* };
}

impl_media_file!(MediaFileType, AudioType);

// ---------------------------------------------------------------------------
// DeviceTrait — types with device fields
// ---------------------------------------------------------------------------

pub trait DeviceTrait: ReferenceableTrait {
    fn get_parameters(&self) -> Option<&DeviceParametersElementType>;
    fn get_enabled(&self) -> Option<&BoolParameterType>;
    fn get_state(&self) -> Option<&FileReferenceType>;
    fn get_device_id(&self) -> Option<&str>;
    fn get_device_name(&self) -> &str;
    fn get_device_role(&self) -> &DeviceRoleType;
    fn get_device_vendor(&self) -> Option<&str>;
    fn get_loaded(&self) -> Option<bool>;
}

macro_rules! impl_device {
    ($($type:ty),* $(,)?) => { $(
        impl DeviceTrait for $type {
            fn get_parameters(&self) -> Option<&DeviceParametersElementType> { self.parameters.as_ref() }
            fn get_enabled(&self) -> Option<&BoolParameterType> { self.enabled.as_ref() }
            fn get_state(&self) -> Option<&FileReferenceType> { self.state.as_ref() }
            fn get_device_id(&self) -> Option<&str> { self.device_id.as_deref() }
            fn get_device_name(&self) -> &str { &self.device_name }
            fn get_device_role(&self) -> &DeviceRoleType { &self.device_role }
            fn get_device_vendor(&self) -> Option<&str> { self.device_vendor.as_deref() }
            fn get_loaded(&self) -> Option<bool> { self.loaded }
        }
    )* };
}

impl_device!(
    DeviceType,
    BuiltinDeviceType,
    EqualizerType,
    CompressorType,
    NoiseGateType,
    LimiterType,
    PluginDyn,
    AuPluginType,
);

impl DeviceTrait for PluginType {
    fn get_parameters(&self) -> Option<&DeviceParametersElementType> {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_parameters(),
        }
    }
    fn get_enabled(&self) -> Option<&BoolParameterType> {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_enabled(),
        }
    }
    fn get_state(&self) -> Option<&FileReferenceType> {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_state(),
        }
    }
    fn get_device_id(&self) -> Option<&str> {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_device_id(),
        }
    }
    fn get_device_name(&self) -> &str {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_device_name(),
        }
    }
    fn get_device_role(&self) -> &DeviceRoleType {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_device_role(),
        }
    }
    fn get_device_vendor(&self) -> Option<&str> {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_device_vendor(),
        }
    }
    fn get_loaded(&self) -> Option<bool> {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_loaded(),
        }
    }
}

// ---------------------------------------------------------------------------
// PluginTrait — types with plugin_version
// ---------------------------------------------------------------------------

pub trait PluginTrait: DeviceTrait {
    fn get_plugin_version(&self) -> Option<&str>;
}

impl PluginTrait for PluginDyn {
    fn get_plugin_version(&self) -> Option<&str> {
        self.plugin_version.as_deref()
    }
}

impl PluginTrait for AuPluginType {
    fn get_plugin_version(&self) -> Option<&str> {
        self.plugin_version.as_deref()
    }
}

impl PluginTrait for PluginType {
    fn get_plugin_version(&self) -> Option<&str> {
        match self {
            Self::Vst2Plugin(value)
            | Self::Vst3Plugin(value)
            | Self::ClapPlugin(value)
            | Self::AuPlugin(value) => value.get_plugin_version(),
        }
    }
}

// ---------------------------------------------------------------------------
// BuiltinDeviceTrait — marker for builtin devices
// ---------------------------------------------------------------------------

pub trait BuiltinDeviceTrait: DeviceTrait {}

impl BuiltinDeviceTrait for BuiltinDeviceType {}
impl BuiltinDeviceTrait for EqualizerType {}
impl BuiltinDeviceTrait for CompressorType {}
impl BuiltinDeviceTrait for NoiseGateType {}
impl BuiltinDeviceTrait for LimiterType {}

// ---------------------------------------------------------------------------
// PointTrait — types with time
// ---------------------------------------------------------------------------

pub trait PointTrait {
    fn get_time(&self) -> &str;
}

macro_rules! impl_point {
    ($($type:ty),* $(,)?) => { $(
        impl PointTrait for $type {
            fn get_time(&self) -> &str { &self.time }
        }
    )* };
}

impl_point!(
    PointDyn,
    BoolPointType,
    EnumPointType,
    RealPointType,
    TimeSignaturePointType,
);

impl PointTrait for PointType {
    fn get_time(&self) -> &str {
        match self {
            Self::RealPoint(value) => value.get_time(),
            Self::EnumPoint(value) | Self::IntegerPoint(value) => value.get_time(),
            Self::BoolPoint(value) => value.get_time(),
            Self::TimeSignaturePoint(value) => value.get_time(),
        }
    }
}
