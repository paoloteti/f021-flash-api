#![allow(bad_style)]
#![allow(dead_code)]
///
/// Based on to:
///   Hercules ARM Safety MCUs - F021 Flash API
///   v2.01.01 Build(000830)
///   Build Date: 2014-10-21
///
extern crate libc;
use libc::types::common::c99::*;

pub mod f021;
use f021::*;

pub type FlashStatusType = uint32_t;

#[repr(C)]
pub enum FlashProgrammingCommands {
    /// This is the default mode for the command
    /// and will auto generate the ECC for the provided data buffer
    AutoEccGeneration,
    /// Command will only process the data buffer
    DataOnly,
    /// Command will only process the ecc buffer
    EccOnly,
    /// Command will process data and ecc buffers
    DataAndEcc,
}

#[repr(C)]
pub enum Status {
   Status_Success=0,           /* Function completed successfully */
   Status_FsmBusy,             /* FSM is Busy */
   Status_FsmReady,            /* FSM is Ready */
   Error_Fail,                 /* Generic Function Fail code */
   Error_NullPointer,          /* One of the pointer parameters is a null pointer */
   Error_InvalidCommand,       /* Command used is invalid for the function called */
   Error_InvalidEccAddress,    /* Returned if the ECC Address given to a function is invalid for that function */
   Error_OtpChecksumMismatch,  /* Returned if OTP checksum does not match expected value */
   Error_InvalidHclkValue,     /* Returned if FClk is above max FClk value - FClk is a calculated from HClk and
                                       RWAIT/EWAIT */
   Error_InvalidBank,          /* Returned if the specified bank does not exist */
   Error_InvalidAddress,       /* Returned if the specified Address does not exist in Flash or OTP */
   Error_InvalidReadMode,      /* Returned if the specified read mode does not exist */
   Error_AsyncIncorrectDataBufferLength,   /* Returned if Data buffer size specified exceeds Data bank width */
   Error_AsyncIncorrectEccBufferLength,    /* Returned if ECC buffer size specified exceeds ECC bank width */
   Error_AsyncDataEccBufferLengthMismatch, /* Returned if Data buffer size either is not 64bit aligned or Data
                                                   length exceeds amount ECC supplied */
   Error_FeatureNotAvailable  /* FMC feature is not available on this device */
}

#[repr(C)]
enum ApiProductionStatus {
   Alpha_Internal,          /* For internal TI use only.  Not intended to be used by customers */
   Alpha,                   /* Early Engineering release.  May not be functionally complete */
   Beta_Internal,           /* For internal TI use only.  Not intended to be used by customers */
   Beta,                    /* Functionally complete, to be used for testing and validation */
   Production               /* Fully validated, functionally complete, ready for production use */
}

#[repr(C)]
pub struct LibraryInfo{
   ApiMajorVersion: u8,
   ApiMinorVersion: u8,
   ApiRevision: u8,
   ApiProductionStatus: ApiProductionStatus,
   ApiBuildNumber: u32,
   ApiTechnologyType: u8,
   ApiTechnologyRevision: u8,
   ApiEndianness: u8,
   ApiCompilerVersion: u32,
}

#[link(name = "flash")]
extern {
    #[link_name = "Fapi_getLibraryInfo"]
    pub fn getLibraryInfo() -> LibraryInfo;

    #[link_name = "Fapi_getDeviceInfo"]
    pub fn getDeviceInfo() -> DeviceInfo;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_isAddressEcc"]
    pub fn isAddressEcc(address: u32) -> bool;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_isAddressEEPROM"]
    pub fn isAddressEEPROM(address: u32) -> bool;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_enableAutoEccCalculation"]
    pub fn enableAutoEccCalculation() -> Status;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_disableAutoEccCalculation"]
    pub fn disableAutoEccCalculation() -> Status;

    #[link_name = "Fapi_getNumberOfBankSectors"]
    pub fn getNumberOfBankSectors(Bank: u32) -> u32;

    #[link_name = "Fapi_flushPipeline"]
    pub fn flushPipeline();

    #[link_name = "Fapi_calculateFletcherChecksum"]
    pub fn calculateFletcherChecksum(addr: uint32_t, len: uint32_t) -> uint32_t;

    #[link_name = "Fapi_calculateEcc"]
    pub fn calculateEcc(Address:uint32_t, Data:uint64_t) -> uint8_t;

    // Programming Commands
    #[link_name = "issueProgrammingCommand"]
    pub fn issueProgrammingCommand(StartAddress: *const uint32_t,
                                   DataBuffer: *const uint8_t,
                                   DataBufferSizeInBytes: uint8_t,
                                   EccBuffer: *const uint8_t,
                                   EccBufferSizeInBytes: uint8_t,
                                   mode: FlashProgrammingCommands) -> Status;

}


