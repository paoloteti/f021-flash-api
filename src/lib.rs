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
enum ApiProductionStatusType {
   Alpha_Internal,          /* For internal TI use only.  Not intended to be used by customers */
   Alpha,                   /* Early Engineering release.  May not be functionally complete */
   Beta_Internal,           /* For internal TI use only.  Not intended to be used by customers */
   Beta,                    /* Functionally complete, to be used for testing and validation */
   Production               /* Fully validated, functionally complete, ready for production use */
}

#[repr(C)]
pub struct LibraryInfo{
   ApiMajorVersion: uint8_t,
   ApiMinorVersion: uint8_t,
   ApiRevision: uint8_t,
   ApiProductionStatus: ApiProductionStatusType,
   ApiBuildNumber: uint32_t,
   ApiTechnologyType: uint8_t,
   ApiTechnologyRevision: uint8_t,
   ApiEndianness: uint8_t,
   ApiCompilerVersion: uint32_t,
}

#[repr(C)]
pub struct FlashStatusWordType {
    StatusWord: [uint32_t; 4]
}

#[repr(C)]
pub enum FlashReadMarginModeType {
   NormalRead = 0x0,
   RM0        = 0x1,
   RM1        = 0x2,
}

#[cfg(target_endian = "little")]
#[repr(C)]
pub struct DeviceInfo {
   NumberOfBanks: uint16_t,
   Reserved: uint16_t,
   DeviceMemorySize: uint16_t,
   DevicePackage: uint16_t,
   AsicId: uint32_t,
   LotNumber: uint32_t,
   WaferNumber: uint16_t,
   FlowCheck: uint16_t,
   WaferYCoordinate: uint16_t,
   WaferXCoordinate: uint16_t,
}

#[cfg(target_endian = "big")]
#[repr(C)]
pub struct DeviceInfo {
   Reserved: uint16_t,
   NumberOfBanks: uint16_t,
   DevicePackage: uint16_t,
   DeviceMemorySize: uint16_t,
   AsicId :uint32_t,
   LotNumber: uint32_t,
   FlowCheck: uint16_t,
   WaferNumber: uint16_t,
   WaferXCoordinate: uint16_t,
   WaferYCoordinate: uint16_t,
}


#[link(name = "flash")]
extern {
    #[link_name = "Fapi_getLibraryInfo"]
    pub fn getLibraryInfo() -> LibraryInfo;

    #[link_name = "Fapi_getDeviceInfo"]
    pub fn getDeviceInfo() -> DeviceInfo;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_isAddressEcc"]
    pub fn isAddressEcc(address: uint32_t) -> bool;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_isAddressEEPROM"]
    pub fn isAddressEEPROM(address: uint32_t) -> bool;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_enableAutoEccCalculation"]
    pub fn enableAutoEccCalculation() -> Status;

    #[cfg(flash_controller="L2FMC")]
    #[link_name = "Fapi_disableAutoEccCalculation"]
    pub fn disableAutoEccCalculation() -> Status;

    #[link_name = "Fapi_getNumberOfBankSectors"]
    pub fn getNumberOfBankSectors(Bank: uint32_t) -> uint32_t;

    #[link_name = "Fapi_doBlankCheck"]
    pub fn doBlankCheck(StartAddress: *const uint32_t,
                        Length: uint32_t,
                        FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_doMarginRead"]
    pub fn doMarginRead(StartAddress: *const uint32_t,
                        ReadBuffer: *const uint32_t,
                        Length: uint32_t,
                        oReadMode: FlashReadMarginModeType) -> Status;

    #[link_name = "Fapi_doVerify"]
    pub fn doVerify(StartAddress: *const uint32_t,
                    Length: uint32_t,
                    CheckValueBuffer: *const uint32_t,
                    FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_calculatePsa"]
    pub fn calculatePsa(StartAddress: *const uint32_t,
                        Length: uint32_t,
                        PsaSeed: uint32_t,
                        oReadMode: FlashReadMarginModeType) -> uint32_t;

    #[link_name = "Fapi_doPsaVerify"]
    pub fn doPsaVerify(StartAddress: *const uint32_t,
                       Length: uint32_t,
                       PsaValue: uint32_t,
                       FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_doBlankCheckByByte"]
    pub fn doBlankCheckByByte(StartAddress: *const uint8_t,
                              Length: uint32_t,
                              FlashStatusWord: *const FlashStatusWordType) -> Status;


    #[link_name = "Fapi_doMarginReadByByte"]
    pub fn doMarginReadByByte(StartAddress: *const uint8_t,
                              ReadBuffer: *const uint8_t,
                              Length: uint32_t,
                              oReadMode: FlashReadMarginModeType) -> Status;

    #[link_name = "Fapi_doVerifyByByte"]
    pub fn doVerifyByByte(StartAddress: *const uint8_t,
                          Length: uint32_t,
                          CheckValueBuffer: *const uint8_t,
                          FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_flushPipeline"]
    pub fn flushPipeline();

    #[link_name = "Fapi_calculateFletcherChecksum"]
    pub fn calculateFletcherChecksum(addr: uint32_t, len: uint32_t) -> uint32_t;

    #[link_name = "Fapi_calculateEcc"]
    pub fn calculateEcc(Address:uint32_t, Data:uint64_t) -> uint8_t;

    // Programming Commands
    #[link_name = "Fapi_issueProgrammingCommand"]
    pub fn issueProgrammingCommand(StartAddress: *const uint32_t,
                                   DataBuffer: *const uint8_t,
                                   DataBufferSizeInBytes: uint8_t,
                                   EccBuffer: *const uint8_t,
                                   EccBufferSizeInBytes: uint8_t,
                                   mode: FlashProgrammingCommands) -> Status;

    #[link_name = "Fapi_issueProgrammingCommandForEccAddresses"]
    pub fn  issueProgrammingCommandForEccAddresses(StartAddress: *const uint32_t,
                                                   EccBuffer: *const uint8_t,
                                                   EccBufferSizeInBytes: uint8_t) -> Status;
}


