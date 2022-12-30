#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CronJob {
    #[prost(string, tag = "1")]
    pub schedule: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonCronJob {
    #[prost(bool, tag = "1")]
    pub repeating: bool,
    #[prost(uint64, tag = "2")]
    pub repeated_every: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(uint64, tag = "1")]
    pub id1: u64,
    #[prost(uint64, tag = "2")]
    pub id2: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStoredData {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<Uuid>,
    #[prost(uint64, optional, tag = "2")]
    pub last_updated: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub last_tick: ::core::option::Option<u64>,
    #[prost(uint64, tag = "4")]
    pub next_tick: u64,
    #[prost(enumeration = "JobType", tag = "5")]
    pub job_type: i32,
    #[prost(uint32, tag = "8")]
    pub count: u32,
    #[prost(bytes = "vec", tag = "9")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "10")]
    pub ran: bool,
    #[prost(bool, tag = "11")]
    pub stopped: bool,
    #[prost(oneof = "job_stored_data::Job", tags = "6, 7")]
    pub job: ::core::option::Option<job_stored_data::Job>,
}
/// Nested message and enum types in `JobStoredData`.
pub mod job_stored_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Job {
        #[prost(message, tag = "6")]
        CronJob(super::CronJob),
        #[prost(message, tag = "7")]
        NonCronJob(super::NonCronJob),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobIdAndNotification {
    #[prost(message, optional, tag = "1")]
    pub job_id: ::core::option::Option<Uuid>,
    #[prost(message, optional, tag = "2")]
    pub notification_id: ::core::option::Option<Uuid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationData {
    #[prost(message, optional, tag = "1")]
    pub job_id: ::core::option::Option<JobIdAndNotification>,
    #[prost(enumeration = "JobState", repeated, tag = "2")]
    pub job_states: ::prost::alloc::vec::Vec<i32>,
    #[prost(bytes = "vec", tag = "3")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationIdAndState {
    #[prost(message, optional, tag = "1")]
    pub notification_id: ::core::option::Option<Uuid>,
    #[prost(enumeration = "JobState", tag = "2")]
    pub job_state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobAndNextTick {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<Uuid>,
    #[prost(enumeration = "JobType", tag = "2")]
    pub job_type: i32,
    #[prost(uint64, tag = "3")]
    pub next_tick: u64,
    #[prost(uint64, optional, tag = "4")]
    pub last_tick: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfUuids {
    #[prost(message, repeated, tag = "1")]
    pub uuids: ::prost::alloc::vec::Vec<Uuid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobAndNotifications {
    #[prost(message, optional, tag = "1")]
    pub job_id: ::core::option::Option<Uuid>,
    #[prost(message, repeated, tag = "2")]
    pub notification_ids: ::prost::alloc::vec::Vec<Uuid>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOfJobsAndNotifications {
    #[prost(message, repeated, tag = "1")]
    pub job_and_notifications: ::prost::alloc::vec::Vec<JobAndNotifications>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobState {
    Stop = 0,
    Scheduled = 1,
    Started = 2,
    Done = 3,
    Removed = 4,
}
impl JobState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobState::Stop => "Stop",
            JobState::Scheduled => "Scheduled",
            JobState::Started => "Started",
            JobState::Done => "Done",
            JobState::Removed => "Removed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Stop" => Some(Self::Stop),
            "Scheduled" => Some(Self::Scheduled),
            "Started" => Some(Self::Started),
            "Done" => Some(Self::Done),
            "Removed" => Some(Self::Removed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobType {
    Cron = 0,
    Repeated = 1,
    OneShot = 2,
}
impl JobType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobType::Cron => "Cron",
            JobType::Repeated => "Repeated",
            JobType::OneShot => "OneShot",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Cron" => Some(Self::Cron),
            "Repeated" => Some(Self::Repeated),
            "OneShot" => Some(Self::OneShot),
            _ => None,
        }
    }
}
