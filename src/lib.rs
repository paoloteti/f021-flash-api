#![allow(bad_style)]
#![allow(dead_code)]
///
/// Based on to:
///   Hercules ARM Safety MCUs - F021 Flash API
///   v2.01.01 Build(000830)
///   Build Date: 2014-10-21
///

pub type FlashStatusType = u32;

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
    /// For internal TI use only. Not intended to be used by customers
    Alpha_Internal,
    /// Early Engineering release. May not be functionally complete
    Alpha,
    /// For internal TI use only. Not intended to be used by customers
    Beta_Internal,
    /// Functionally complete, to be used for testing and validation
    Beta,
    /// Fully validated, functionally complete, ready for production use
    Production,
}

#[repr(C)]
pub struct LibraryInfo{
   ApiMajorVersion: u8,
   ApiMinorVersion: u8,
   ApiRevision: u8,
   ApiProductionStatus: ApiProductionStatusType,
   ApiBuildNumber: u32,
   ApiTechnologyType: u8,
   ApiTechnologyRevision: u8,
   ApiEndianness: u8,
   ApiCompilerVersion: u32,
}

#[repr(C)]
pub struct FlashStatusWordType {
    StatusWord: [u32; 4]
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
   NumberOfBanks: u16,
   Reserved: u16,
   DeviceMemorySize: u16,
   DevicePackage: u16,
   AsicId: u32,
   LotNumber: u32,
   WaferNumber: u16,
   FlowCheck: u16,
   WaferYCoordinate: u16,
   WaferXCoordinate: u16,
}

#[cfg(target_endian = "big")]
#[repr(C)]
pub struct DeviceInfo {
   Reserved: u16,
   NumberOfBanks: u16,
   DevicePackage: u16,
   DeviceMemorySize: u16,
   AsicId :u32,
   LotNumber: u32,
   FlowCheck: u16,
   WaferNumber: u16,
   WaferXCoordinate: u16,
   WaferYCoordinate: u16,
}

#[repr(C)]
enum FlashBankTechType {
    FLEP = 0,
    FLEE = 1,
    FLES = 2,
    FLHV = 3
}

#[repr(C)]
pub struct FlashBankSectorsType {
    FlashBankTech: FlashBankTechType,
    NumberOfSectors: u32,
    BankStartAddress: u32,
    SectorSizes: [u16; 16],
}

#[repr(C)]
pub enum FlashBankType {
   Bank0 = 0,
   Bank1 = 1,
   Bank2 = 2,
   Bank3 = 3,
   Bank4 = 4,
   Bank5 = 5,
   Bank6 = 6,
   Bank7 = 7,
}



#[link(name = "flash")]
extern {
    #[link_name = "Fapi_getLibraryInfo"]
    pub fn getLibraryInfo() -> LibraryInfo;

    #[link_name = "Fapi_getDeviceInfo"]
    pub fn getDeviceInfo() -> DeviceInfo;

    #[link_name = "Fapi_getBankSectors"]
    pub fn getBankSectors(Bank: FlashBankType,
                          FlashBankSectors: *const FlashBankSectorsType);

    #[link_name = "Fapi_getNumberOfBankSectors"]
    pub fn getNumberOfBankSectors(Bank: u32) -> u32;

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

    #[link_name = "Fapi_doBlankCheck"]
    pub fn doBlankCheck(StartAddress: *const u32,
                        Length: u32,
                        FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_doMarginRead"]
    pub fn doMarginRead(StartAddress: *const u32,
                        ReadBuffer: *const u32,
                        Length: u32,
                        oReadMode: FlashReadMarginModeType) -> Status;

    #[link_name = "Fapi_doVerify"]
    pub fn doVerify(StartAddress: *const u32,
                    Length: u32,
                    CheckValueBuffer: *const u32,
                    FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_calculatePsa"]
    pub fn calculatePsa(StartAddress: *const u32,
                        Length: u32,
                        PsaSeed: u32,
                        oReadMode: FlashReadMarginModeType) -> u32;

    #[link_name = "Fapi_doPsaVerify"]
    pub fn doPsaVerify(StartAddress: *const u32,
                       Length: u32,
                       PsaValue: u32,
                       FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_doBlankCheckByByte"]
    pub fn doBlankCheckByByte(StartAddress: *const u8,
                              Length: u32,
                              FlashStatusWord: *const FlashStatusWordType) -> Status;


    #[link_name = "Fapi_doMarginReadByByte"]
    pub fn doMarginReadByByte(StartAddress: *const u8,
                              ReadBuffer: *const u8,
                              Length: u32,
                              ReadMode: FlashReadMarginModeType) -> Status;

    #[link_name = "Fapi_doVerifyByByte"]
    pub fn doVerifyByByte(StartAddress: *const u8,
                          Length: u32,
                          CheckValueBuffer: *const u8,
                          FlashStatusWord: *const FlashStatusWordType) -> Status;

    #[link_name = "Fapi_flushPipeline"]
    pub fn flushPipeline();

    #[link_name = "Fapi_calculateFletcherChecksum"]
    pub fn calculateFletcherChecksum(addr: u32, len: u32) -> u32;

    #[link_name = "Fapi_calculateEcc"]
    pub fn calculateEcc(Address:u32, Data: u64) -> u8;

    // Programming Commands
    #[link_name = "Fapi_issueProgrammingCommand"]
    pub fn issueProgrammingCommand(StartAddress: *const u32,
                                   DataBuffer: *const u8,
                                   DataBufferSizeInBytes: u8,
                                   EccBuffer: *const u8,
                                   EccBufferSizeInBytes: u8,
                                   mode: FlashProgrammingCommands) -> Status;

    #[link_name = "Fapi_issueProgrammingCommandForEccAddresses"]
    pub fn  issueProgrammingCommandForEccAddresses(StartAddress: *const u32,
                                                   EccBuffer: *const u8,
                                                   EccBufferSizeInBytes: u8) -> Status;
}


