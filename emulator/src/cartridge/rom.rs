use crate::common::constants::{CHR_ROM_PAGE_SIZE, NES_HEADER_SIZE, NES_TAG, NES_TRAINER_SIZE, PRG_ROM_PAGE_SIZE};
use crate::common::types::Mirroring;
use crate::common::errors::EmulatorError;

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub mirroring: Mirroring,
}

impl Rom {
    pub fn new(bytes: &Vec<u8>) -> Result<Self, EmulatorError> {
        Self::validate_file_format(bytes)?;
        let (mirroring, mapper) = Self::parse_control_bytes(bytes)?;
        let (prg_rom, chr_rom) = Self::extract_rom_sections(bytes)?;

        Ok(Rom { prg_rom, chr_rom, mapper, mirroring })
    }

    pub fn default() -> Self {
        Rom {
            prg_rom: vec![0; PRG_ROM_PAGE_SIZE],
            chr_rom: vec![0; CHR_ROM_PAGE_SIZE],
            mapper: 0,
            mirroring: Mirroring::Horizontal,
        }
    }

    fn validate_file_format(bytes: &[u8]) -> Result<(), EmulatorError> {
        if bytes.starts_with(&NES_TAG) {
            Ok(())
        } else {
            Err(EmulatorError::InvalidNesFile)
        }
    }

    fn parse_control_bytes(bytes: &[u8]) -> Result<(Mirroring, u8), EmulatorError> {
        let ctrl_byte_1 = bytes[6];
        let ctrl_byte_2= bytes[7];
        if ctrl_byte_2 & 0b0000_1111 != 0 {
            return Err(EmulatorError::InvalidNesFile);
        }

        let mirroring = match (ctrl_byte_1 & 0b0000_1000 != 0, ctrl_byte_1 & 0b0000_0001 != 0) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let mapper_upper = ctrl_byte_2 & 0b1111_0000;
        let mapper_lower = ctrl_byte_1 & 0b1111_0000;
        let mapper = mapper_upper | mapper_lower;

        Ok((mirroring, mapper))
    }

    fn extract_rom_sections(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), EmulatorError> {
        let prg_rom_size = bytes[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = bytes[5] as usize * CHR_ROM_PAGE_SIZE;
        let has_trainer = bytes[6] & 0b0000_0100 != 0;
        let prg_rom_start = NES_HEADER_SIZE + if has_trainer { NES_TRAINER_SIZE } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let prg_rom = bytes[prg_rom_start..prg_rom_start + prg_rom_size].to_vec();
        let chr_rom = bytes[chr_rom_start..chr_rom_start + chr_rom_size].to_vec();

        Ok((prg_rom, chr_rom))
    }
}