use number_types::d8_type::d8;
use number_types::d16_type::d16;
use number_types::a16_type::a16;
use std::num::Wrapping;

type CartRomBank0 = [d8; 0x3eb0];
type CartRomBankN = [d8; 0x4000];
type BackgroundMapData = [d8; 0x400];
type InternalRamBank = [d8; 0x1000];

/*
http://gameboy.mongenel.com/dmg/asmmemmap.html :

GameBoy Memory Areas
$FFFF	Interrupt Enable Flag
$FF80-$FFFE	Zero Page - 127 bytes
$FF00-$FF7F	Hardware I/O Registers
$FEA0-$FEFF	Unusable Memory
$FE00-$FE9F	OAM - Object Attribute Memory
$E000-$FDFF	Echo RAM - Reserved, Do Not Use
$D000-$DFFF	Internal RAM - Bank 1-7 (switchable - CGB only)
$C000-$CFFF	Internal RAM - Bank 0 (fixed)
$A000-$BFFF	Cartridge RAM (If Available)
$9C00-$9FFF	BG Map Data 2
$9800-$9BFF	BG Map Data 1
$8000-$97FF	Character RAM
$4000-$7FFF	Cartridge ROM - Switchable Banks 1-xx
$0150-$3FFF	Cartridge ROM - Bank 0 (fixed)
$0100-$014F	Cartridge Header Area
$0000-$00FF	Restart and Interrupt Vectors
*/

pub struct Memory {
    restart_and_int_vectors: [d8; 0x100],
    cartridge_header: [d8; 0x50],
    cart_rom_bank_0: CartRomBank0,
    other_cart_rom_banks: Vec<CartRomBankN>,
    active_rom_bank_index: usize,
    character_ram: [d8; 0x800],
    background_data_0: BackgroundMapData,
    background_data_1: BackgroundMapData,
    cart_ram: Option<[d8; 0x2000]>,
    internal_ram_bank_0: InternalRamBank,
    other_internal_ram_banks: Vec<InternalRamBank>,
    active_ram_bank_index: usize,
    object_attribute_memory: [d8; 0xa0],
    hardware_io_regs: [d8; 0x80],
    enable_interrupt_flag: d8,
}

impl Memory {
    pub fn new_zeros() -> Self {
        Self {
            restart_and_int_vectors: [d8::ZERO; 0x100],
            cartridge_header: [d8::ZERO; 0x50],
            cart_rom_bank_0: [d8::ZERO; 0x3eb0],
            other_cart_rom_banks: vec![[d8::ZERO; 0x4000]],
            active_rom_bank_index: 0,
            character_ram: [d8::ZERO; 0x800],
            background_data_0: [d8::ZERO; 0x400],
            background_data_1: [d8::ZERO; 0x400],
            cart_ram: None,
            internal_ram_bank_0: [d8::ZERO; 0x1000],
            other_internal_ram_banks: vec![[d8::ZERO; 0x1000]],
            active_ram_bank_index: 0,
            object_attribute_memory: [d8::ZERO; 0xa0],
            hardware_io_regs: [d8::ZERO; 0x80],
            enable_interrupt_flag: d8::ZERO,
        }
    }
    
    pub fn read_d8(&self, a16(Wrapping(idx)): a16) -> Option<d8> {
        let idx = idx as usize;
        match idx {
            0x0000 ... 0x00ff => Some(self.restart_and_int_vectors[idx]),
            0x0100 ... 0x014f => Some(self.cartridge_header[idx - 0x0100]),
            0x0150 ... 0x3fff => Some(self.cart_rom_bank_0[idx - 0x0150]),
            0x4000 ... 0x7fff => Some(self.other_cart_rom_banks[
                self.active_rom_bank_index
            ][idx - 0x4000]),
            0x8000 ... 0x97ff => Some(self.character_ram[idx - 0x8000]),
            0x9800 ... 0x9bff => Some(self.background_data_0[idx - 0x9800]),
            0x9c00 ... 0x9fff => Some(self.background_data_1[idx - 0x9c00]),
            0xa000 ... 0xbfff => self.cart_ram.as_ref().and_then(|mem| {
                Some(mem[idx - 0xa000])
            }),
            0xc000 ... 0xcfff => Some(self.internal_ram_bank_0[idx - 0xc000]),
            0xd000 ... 0xdfff => Some(self.other_internal_ram_banks[
                self.active_ram_bank_index
            ][idx - 0xd000]),
            0xe000 ... 0xfdff => None,
            0xfe00 ... 0xfe9f => Some(self.object_attribute_memory[idx - 0xfe00]),
            0xfea0 ... 0xfeff => None,
            0xff00 ... 0xff7f => Some(self.hardware_io_regs[idx - 0xff00]),
            0xff80 ... 0xfffe => Some(d8::ZERO),
            0xffff => Some(self.enable_interrupt_flag),
            _ => unreachable!(),
        }
    }

    pub fn put_d8(&mut self, a16(Wrapping(idx)): a16, val: d8) -> Option<()> {
        let idx = idx as usize;
        match idx {
            0x0000 ... 0x00ff => Some(self.restart_and_int_vectors[idx] = val),
            0x0100 ... 0x014f => None,
            0x0150 ... 0x3fff => None,
            0x4000 ... 0x7fff => None,
            0x8000 ... 0x97ff => Some(self.character_ram[idx - 0x8000] = val),
            0x9800 ... 0x9bff => Some(self.background_data_0[idx - 0x9800] = val),
            0x9c00 ... 0x9fff => Some(self.background_data_1[idx - 0x9c00] = val),
            0xa000 ... 0xbfff => self.cart_ram.as_mut().and_then(|mem| {
                Some(mem[idx - 0xa000] = val)
            }),
            0xc000 ... 0xcfff => Some(self.internal_ram_bank_0[idx - 0xc000] = val),
            0xd000 ... 0xdfff => Some(self.other_internal_ram_banks[
                self.active_ram_bank_index
            ][idx - 0xd000] = val),
            0xe000 ... 0xfdff => None,
            0xfe00 ... 0xfe9f => Some(self.object_attribute_memory[idx - 0xfe00] = val),
            0xfea0 ... 0xfeff => None,
            0xff00 ... 0xff7f => Some(self.hardware_io_regs[idx - 0xff00] = val),
            0xff80 ... 0xfffe => None,
            0xffff => Some(self.enable_interrupt_flag = val),
            _ => unreachable!(),
        }
    }

    pub fn read_d16(&self, idx: a16) -> Option<d16> {
        Some([
            self.read_d8(idx).unwrap_or(d8::ZERO),
            self.read_d8(idx + 1).unwrap_or(d8::ZERO)
        ].into())
    }

    pub fn put_d16(&mut self, idx: a16, val: d16) -> Option<()> {
        let val: [d8; 2] = val.into();
        let (lsb, msb) = (val[0], val[1]);
        let lsb: Option<()> = self.put_d8(idx, lsb);
        let msb: Option<()> = self.put_d8(idx + 1, msb);
        lsb.and(msb)
    }
}
