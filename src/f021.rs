


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

struct f021 {

}
