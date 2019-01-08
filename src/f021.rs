/// Specifies the bit mask for determining all address bits exclusive of
/// the offest imposed by the memory map register
const PROGRAM_ADDRESS_MASK: u32 = 0x07FF_FFFF;

/// Specifies the Offset to the TI OTP
const PROGRAM_TIOTP_OFFSET: u32 = 0xF008_0000;

const FLASH_MAP_BEGIN: u32 = 0x0000_0000;
const FLASH_MAP_END: u32 = 0x00FF_FFFF;
const OTP_MAP_BEGIN: u32 = 0xF000_0000;
const OTP_MAP_END: u32 = 0xF000_FFFF;
const OTPECC_MAP_BEGIN: u32 = 0xF004_0000;
const OTPECC_MAP_END: u32 = 0xF004_1FFF;
const EEPROMECC_MAP_BEGIN: u32 = 0xF010_0000;
const EEPROMECC_MAP_END: u32 = 0xF01F_FFFF;
const EEPROM_MAP_BEGIN: u32 = 0xF020_0000;
const EEPROM_MAP_END: u32 = 0xF03F_FFFF;
const FLASHECC_MAP_BEGIN: u32 = 0xF040_0000;
const FLASHECC_MAP_END: u32 = 0xF04F_FFFF;

const CPU0_REGISTER_ADDRESS: u32 = 0xFFF8_7000;

// TI-OTP Constants
const TIOTP_PER_BANK_SIZE: u32 = 0x2000;
const TIOTP_SETTINGS_BASE: u32 = 0x150;
const TIOTP_BANK_SECTOR_OFFSET: u32 = 0x158;

/// Address for direct access to the TI OTP memory
const TIOTP_BASE_ADDRESS: u32 = PROGRAM_TIOTP_OFFSET + TIOTP_SETTINGS_BASE;
