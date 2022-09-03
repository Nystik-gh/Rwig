use serde::{Deserialize, Serialize};
use windows::Win32::Media::Audio::{WAVEFORMATEX, WAVEFORMATEXTENSIBLE, WAVEFORMATEXTENSIBLE_0};

pub const WAVE_FORMAT_PCM: u16 = 0x0001;
pub const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;

pub const WAVE_FORMAT_EXTENSIBLE: u16 = 0xfffe;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl From<windows::core::GUID> for GUID {
    fn from(guid: windows::core::GUID) -> Self {
        let windows::core::GUID {
            data1,
            data2,
            data3,
            data4,
        } = guid;
        GUID {
            data1,
            data2,
            data3,
            data4,
        }
    }
}

impl Into<windows::core::GUID> for GUID {
    fn into(self) -> windows::core::GUID {
        let GUID {
            data1,
            data2,
            data3,
            data4,
        } = self;
        windows::core::GUID::from_values(data1, data2, data3, data4)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum FormatTag {
    WaveFormatFloat,
    WaveFormatPcm,
    WaveFormatExtensible,
    Other(u16),
}

impl From<u16> for FormatTag {
    fn from(tag: u16) -> Self {
        match tag {
            WAVE_FORMAT_PCM => FormatTag::WaveFormatPcm,
            WAVE_FORMAT_IEEE_FLOAT => FormatTag::WaveFormatFloat,
            WAVE_FORMAT_EXTENSIBLE => FormatTag::WaveFormatExtensible,
            _ => FormatTag::Other(tag),
        }
    }
}

impl From<windows::core::GUID> for FormatTag {
    fn from(guid: windows::core::GUID) -> Self {
        FormatTag::from(guid.data1 as u16)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MixFormatData {
    pub format_tag: FormatTag,
    pub channels: u16,
    pub sample_rate: u32,
    pub byte_transfer_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

impl From<WAVEFORMATEX> for MixFormatData {
    fn from(wf: WAVEFORMATEX) -> Self {
        MixFormatData {
            format_tag: wf.wFormatTag.into(),
            channels: wf.nChannels,
            sample_rate: wf.nSamplesPerSec,
            byte_transfer_rate: wf.nAvgBytesPerSec,
            block_align: wf.nBlockAlign,
            bits_per_sample: wf.wBitsPerSample,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MixFormatSamples {
    pub valid_bits_per_sample: u16,
    pub samples_per_block: u16,
    pub reserved: u16,
}

impl From<WAVEFORMATEXTENSIBLE_0> for MixFormatSamples {
    fn from(samples: WAVEFORMATEXTENSIBLE_0) -> Self {
        unsafe {
            MixFormatSamples {
                valid_bits_per_sample: samples.wValidBitsPerSample,
                samples_per_block: samples.wSamplesPerBlock,
                reserved: samples.wReserved,
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MixFormat {
    pub format: MixFormatData,
    pub samples: Option<MixFormatSamples>,
    pub channel_mask: Option<u32>,
    pub sub_format_guid: Option<GUID>,
    pub sub_format: Option<FormatTag>,
}

impl From<WAVEFORMATEXTENSIBLE> for MixFormat {
    fn from(wf_ext: WAVEFORMATEXTENSIBLE) -> Self {
        MixFormat {
            format: wf_ext.Format.into(),
            samples: Some(wf_ext.Samples.into()),
            channel_mask: Some(wf_ext.dwChannelMask),
            sub_format_guid: Some(wf_ext.SubFormat.into()),
            sub_format: Some(wf_ext.SubFormat.into()),
        }
    }
}

impl From<WAVEFORMATEX> for MixFormat {
    fn from(wf: WAVEFORMATEX) -> Self {
        MixFormat {
            format: wf.into(),
            samples: None,
            channel_mask: None,
            sub_format_guid: None,
            sub_format: None,
        }
    }
}

impl From<*mut WAVEFORMATEXTENSIBLE> for MixFormat {
    fn from(pwf: *mut WAVEFORMATEXTENSIBLE) -> Self {
        let wf_ext = unsafe { *pwf };
        MixFormat {
            format: wf_ext.Format.into(),
            samples: Some(wf_ext.Samples.into()),
            channel_mask: Some(wf_ext.dwChannelMask),
            sub_format_guid: Some(wf_ext.SubFormat.into()),
            sub_format: Some(wf_ext.SubFormat.into()),
        }
    }
}

impl From<*mut WAVEFORMATEX> for MixFormat {
    fn from(pwf: *mut WAVEFORMATEX) -> Self {
        if let FormatTag::WaveFormatExtensible = FormatTag::from((unsafe { *pwf }).wFormatTag) {
            let wf_ext = unsafe { *(pwf as *mut WAVEFORMATEXTENSIBLE) };
            wf_ext.into()
        } else {
            let wf = unsafe { *pwf };
            wf.into()
        }
    }
}
