
use libc::types::common::c99::*;

/// Specifies the bit mask for determining all address bits exclusive of
/// the offest imposed by the memory map register
const F021_PROGRAM_ADDRESS_MASK: uint32_t =  0x07FFFFFF;

/// Specifies the Offset to the TI OTP
const F021_PROGRAM_TIOTP_OFFSET: uint32_t =    0xF0080000;

const F021_FLASH_MAP_BEGIN: uint32_t =     0x00000000;
const F021_FLASH_MAP_END: uint32_t =       0x00FFFFFF;
const F021_OTP_MAP_BEGIN: uint32_t =       0xF0000000;
const F021_OTP_MAP_END: uint32_t =         0xF000FFFF;
const F021_OTPECC_MAP_BEGIN: uint32_t =    0xF0040000;
const F021_OTPECC_MAP_END: uint32_t =      0xF0041FFF;
const F021_EEPROMECC_MAP_BEGIN: uint32_t = 0xF0100000;
const F021_EEPROMECC_MAP_END: uint32_t =   0xF01FFFFF;
const F021_EEPROM_MAP_BEGIN: uint32_t =    0xF0200000;
const F021_EEPROM_MAP_END: uint32_t =      0xF03FFFFF;
const F021_FLASHECC_MAP_BEGIN: uint32_t =  0xF0400000;
const F021_FLASHECC_MAP_END: uint32_t =    0xF04FFFFF;

const F021_CPU0_REGISTER_ADDRESS: uint32_t = 0xFFF87000;

// TI-OTP Constants
const F021_TIOTP_PER_BANK_SIZE: uint32_t      = 0x2000;
const F021_TIOTP_SETTINGS_BASE: uint32_t      = 0x150;
const F021_TIOTP_BANK_SECTOR_OFFSET: uint32_t = 0x158;

/// Address for direct access to the TI OTP memory
const F021_TIOTP_BASE_ADDRESS: uint32_t = F021_PROGRAM_TIOTP_OFFSET + F021_TIOTP_SETTINGS_BASE;
