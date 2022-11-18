use dioxus_core::UiEvent;

pub type MediaEvent = UiEvent<MediaData>;
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct MediaData {}
