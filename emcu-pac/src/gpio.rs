#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DATA Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x04 - Data Output Latch Register"]
    pub dataout: crate::Reg<dataout::DATAOUT_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Output Enable Set Register"]
    pub outenset: crate::Reg<outenset::OUTENSET_SPEC>,
    #[doc = "0x14 - Output Enable Clear Register"]
    pub outenclr: crate::Reg<outenclr::OUTENCLR_SPEC>,
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "DATA Register"]
pub mod data;
#[doc = "DATAOUT register accessor: an alias for `Reg<DATAOUT_SPEC>`"]
pub type DATAOUT = crate::Reg<dataout::DATAOUT_SPEC>;
#[doc = "Data Output Latch Register"]
pub mod dataout;
#[doc = "OUTENSET register accessor: an alias for `Reg<OUTENSET_SPEC>`"]
pub type OUTENSET = crate::Reg<outenset::OUTENSET_SPEC>;
#[doc = "Output Enable Set Register"]
pub mod outenset;
#[doc = "OUTENCLR register accessor: an alias for `Reg<OUTENCLR_SPEC>`"]
pub type OUTENCLR = crate::Reg<outenclr::OUTENCLR_SPEC>;
#[doc = "Output Enable Clear Register"]
pub mod outenclr;
