use alloc::vec::Vec;
use volatile::Volatile;

use crate::println;

pub const ATA_CMD_IDENTIFY: u8 = 0xEC;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FisType {
    RegH2D = 0x27,    // Register FIS - host to device
    RegD2H = 0x34,    // Register FIS - device to host
    DmaAct = 0x39,    // DMA activate FIS - device to host
    DmaSetup = 0x41,  // DMA setup FIS - bidirectional
    Data = 0x46,      // Data FIS - bidirectional
    Bist = 0x58,      // BIST activate FIS - bidirectional
    PioSetup = 0x5F,  // PIO setup FIS - device to host
    DevBits = 0xA1,   // Set device bits FIS - device to host
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisRegH2D {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_REG_H2D
    pub pmport_c: u8,     // Port multiplier (bits 0-3), Reserved (bits 4-6), Command/Control (bit 7)
    pub command: u8,      // Command register
    pub featurel: u8,     // Feature register, 7:0
    
    // DWORD 1
    pub lba0: u8,         // LBA low register, 7:0
    pub lba1: u8,         // LBA mid register, 15:8
    pub lba2: u8,         // LBA high register, 23:16
    pub device: u8,       // Device register
    
    // DWORD 2
    pub lba3: u8,         // LBA register, 31:24
    pub lba4: u8,         // LBA register, 39:32
    pub lba5: u8,         // LBA register, 47:40
    pub featureh: u8,     // Feature register, 15:8
    
    // DWORD 3
    pub countl: u8,       // Count register, 7:0
    pub counth: u8,       // Count register, 15:8
    pub icc: u8,          // Isochronous command completion
    pub control: u8,      // Control register
    
    // DWORD 4
    pub rsv1: [u8; 4],   // Reserved
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisRegD2H {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_REG_D2H
    pub pmport_i: u8,     // Port multiplier (bits 0-3), Reserved (bits 4-5), Interrupt (bit 6), Reserved (bit 7)
    pub status: u8,       // Status register
    pub error: u8,        // Error register
    
    // DWORD 1
    pub lba0: u8,         // LBA low register, 7:0
    pub lba1: u8,         // LBA mid register, 15:8
    pub lba2: u8,         // LBA high register, 23:16
    pub device: u8,       // Device register
    
    // DWORD 2
    pub lba3: u8,         // LBA register, 31:24
    pub lba4: u8,         // LBA register, 39:32
    pub lba5: u8,         // LBA register, 47:40
    pub rsv2: u8,         // Reserved
    
    // DWORD 3
    pub countl: u8,       // Count register, 7:0
    pub counth: u8,       // Count register, 15:8
    pub rsv3: [u8; 2],   // Reserved
    
    // DWORD 4
    pub rsv4: [u8; 4],   // Reserved
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisData {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_DATA
    pub pmport: u8,       // Port multiplier (bits 0-3) and reserved (bits 4-7)
    pub rsv1: [u8; 2],   // Reserved
    
    // DWORD 1 ~ N
    pub data: [u32; 1],   // Payload
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisPioSetup {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_PIO_SETUP
    pub pmport_flags: u8, // Port multiplier and flags
    pub status: u8,       // Status register
    pub error: u8,        // Error register
    
    // DWORD 1
    pub lba0: u8,         // LBA low register, 7:0
    pub lba1: u8,         // LBA mid register, 15:8
    pub lba2: u8,         // LBA high register, 23:16
    pub device: u8,       // Device register
    
    // DWORD 2
    pub lba3: u8,         // LBA register, 31:24
    pub lba4: u8,         // LBA register, 39:32
    pub lba5: u8,         // LBA register, 47:40
    pub rsv2: u8,         // Reserved
    
    // DWORD 3
    pub countl: u8,       // Count register, 7:0
    pub counth: u8,       // Count register, 15:8
    pub rsv3: u8,         // Reserved
    pub e_status: u8,     // New value of status register
    
    // DWORD 4
    pub tc: u16,          // Transfer count
    pub rsv4: [u8; 2],   // Reserved
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisDmaSetup {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_DMA_SETUP
    pub pmport_flags: u8, // Port multiplier and flags
    pub rsved: [u8; 2],  // Reserved
    
    // DWORD 1 & 2
    pub dma_buffer_id: u64, // DMA Buffer Identifier
    
    // DWORD 3
    pub rsvd: u32,       // Reserved
    
    // DWORD 4
    pub dma_buf_offset: u32, // Byte offset into buffer
    
    // DWORD 5
    pub transfer_count: u32, // Number of bytes to transfer
    
    // DWORD 6
    pub resvd: u32,      // Reserved
}

// AHCI device detection constants
pub const SATA_SIG_ATAPI: u32 = 0xEB140101;
pub const SATA_SIG_ATA: u32 = 0x00000101;
pub const SATA_SIG_SEMB: u32 = 0xC33C0101;
pub const SATA_SIG_PM: u32 = 0x96690101;

// Port status and control constants
pub const HBA_PORT_IPM_ACTIVE: u32 = 1;
pub const HBA_PORT_DET_PRESENT: u32 = 3;

#[repr(C)]
pub struct HbaPort {
    pub clb: Volatile<u32>,    // 0x00, command list base address, 1K-byte aligned
    pub clbu: Volatile<u32>,   // 0x04, command list base address upper 32 bits
    pub fb: Volatile<u32>,     // 0x08, FIS base address, 256-byte aligned
    pub fbu: Volatile<u32>,    // 0x0C, FIS base address upper 32 bits
    pub is: Volatile<u32>,     // 0x10, interrupt status
    pub ie: Volatile<u32>,     // 0x14, interrupt enable
    pub cmd: Volatile<u32>,    // 0x18, command and status
    pub rsv0: Volatile<u32>,   // 0x1C, Reserved
    pub tfd: Volatile<u32>,    // 0x20, task file data
    pub sig: Volatile<u32>,    // 0x24, signature
    pub ssts: Volatile<u32>,   // 0x28, SATA status (SCR0:SStatus)
    pub sctl: Volatile<u32>,   // 0x2C, SATA control (SCR2:SControl)
    pub serr: Volatile<u32>,   // 0x30, SATA error (SCR1:SError)
    pub sact: Volatile<u32>,   // 0x34, SATA active (SCR3:SActive)
    pub ci: Volatile<u32>,     // 0x38, command issue
    pub sntf: Volatile<u32>,   // 0x3C, SATA notification (SCR4:SNotification)
    pub fbs: Volatile<u32>,    // 0x40, FIS-based switch control
    pub rsv1: [Volatile<u32>; 11], // 0x44 ~ 0x6F, Reserved
    pub vendor: [Volatile<u32>; 4], // 0x70 ~ 0x7F, vendor specific
}

impl HbaPort {
    pub fn is_device_present(&self) -> bool {
        let ssts = self.ssts.read();
        let det = ssts & 0x0F;
        let ipm = (ssts >> 8) & 0x0F;
        det == HBA_PORT_DET_PRESENT && ipm == HBA_PORT_IPM_ACTIVE
    }

    pub fn get_device_type(&self) -> Option<DeviceType> {
        if !self.is_device_present() {
            return None;
        }

        let sig = self.sig.read();
        match sig {
            SATA_SIG_ATAPI => Some(DeviceType::SATAPI),
            SATA_SIG_ATA => Some(DeviceType::SATA),
            SATA_SIG_SEMB => Some(DeviceType::SEMB),
            SATA_SIG_PM => Some(DeviceType::PM),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceType {
    SATA,    // SATA drive
    SATAPI,  // SATAPI drive
    SEMB,    // Enclosure management bridge
    PM,      // Port multiplier
}

#[repr(C)]
pub struct HbaMem {
    // 0x00 - 0x2B, Generic Host Control
    pub cap: Volatile<u32>,      // 0x00, Host capability
    pub ghc: Volatile<u32>,      // 0x04, Global host control
    pub is: Volatile<u32>,       // 0x08, Interrupt status
    pub pi: Volatile<u32>,       // 0x0C, Port implemented
    pub vs: Volatile<u32>,       // 0x10, Version
    pub ccc_ctl: Volatile<u32>,  // 0x14, Command completion coalescing control
    pub ccc_pts: Volatile<u32>,  // 0x18, Command completion coalescing ports
    pub em_loc: Volatile<u32>,   // 0x1C, Enclosure management location
    pub em_ctl: Volatile<u32>,   // 0x20, Enclosure management control
    pub cap2: Volatile<u32>,     // 0x24, Host capabilities extended
    pub bohc: Volatile<u32>,     // 0x28, BIOS/OS handoff control and status

    // 0x2C - 0x9F, Reserved
    pub rsv: [u8; 0xA0-0x2C],

    // 0xA0 - 0xFF, Vendor specific registers
    pub vendor: [u8; 0x100-0xA0],

    // 0x100 - 0x10FF, Port control registers
    pub ports: [HbaPort; 32],    // 1 ~ 32 ports
}

impl HbaMem {
    pub fn probe_ports(&self) -> Vec<(usize, DeviceType)> {
        let mut devices = Vec::new();
        let pi = self.pi.read();

        // Check each bit in the ports implemented register
        for i in 0..32 {
            if pi & (1 << i) != 0 {
                if let Some(device_type) = self.ports[i].get_device_type() {
                    devices.push((i, device_type));
                }
            }
        }
        devices
    }
}

pub fn init_ahci(abar: usize) -> Option<&'static mut HbaMem> {
    let hba = unsafe { &mut *(abar as *mut HbaMem) };
    
    // Enable AHCI by setting GHC.AE
    let ghc = hba.ghc.read();
    hba.ghc.write(ghc | (1 << 31));
    
    let devices = hba.probe_ports();
    for (port_num, device_type) in devices {
        // Found a device
        match device_type {
            DeviceType::SATA => println!("SATA drive found at port {}", port_num),
            DeviceType::SATAPI => println!("SATAPI drive found at port {}", port_num),
            DeviceType::SEMB => println!("SEMB drive found at port {}", port_num),
            DeviceType::PM => println!("PM drive found at port {}", port_num),
        }
    }

    Some(hba)
}

pub fn read_data() {
    let mut fis = FisRegH2D {
        fis_type: FisType::RegH2D as u8,
        pmport_c: 1 << 7,    // Set the command bit (c=1)
        command: ATA_CMD_IDENTIFY,
        featurel: 0,
        lba0: 0,
        lba1: 0,
        lba2: 0,
        device: 0,           // Master device
        lba3: 0,
        lba4: 0,
        lba5: 0,
        featureh: 0,
        countl: 0,
        counth: 0,
        icc: 0,
        control: 0,
        rsv1: [0; 4],
    };
}
