#![no_std]

//! TODO handle errors


/// Trait representing the interface to the hardware.
/// Intended to abstract the various buses (SPI, MPU 8/9/16/18-bit) from the
/// Controller code.
/// TODO Add support for 16/32-bit words
pub trait Interface {
	fn write_parameters(&mut self, command: u8, data: &[u8]);
    //fn write_memory<I>(&mut self, iterable: I) where I: IntoIterator<Item=u32>;
	fn write_memory(&mut self, data: &[u8]);
	fn read_parameters(&mut self, command: u8, data: &mut [u8]);
    //fn read_memory(&mut self, data: &mut [u32]);
    fn read_memory(&mut self, data: &mut [u8]);
}

pub enum TearingEffect {
	Off,
	VBlankOnly,
	HAndVBlank,
}

// TODO: Implement access "methods" on these types.

#[derive(Copy, Clone, Default, Debug)]
pub struct DisplayIdentification {
	raw: [u8; 3],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct DisplayStatus {
	raw: [u8; 4],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct DisplayPowerMode {
	raw: [u8; 1],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct MADCtl {
	raw: [u8; 1],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct PixelFormat {
	raw: [u8; 1],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct ImageFormat {
	raw: [u8; 1],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct SignalMode {
	raw: [u8; 1],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct SelfDiagnosticResult {
	raw: [u8; 1],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct MemoryAccessControl {
	raw: [u8; 1],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct CtrlDisplay {
	raw: [u8; 1],
}

/// Controller implements the LCD command set and calls on the Interface trait
/// to communicate with the LCD panel.
#[derive(Copy, Clone)]
pub struct Controller<T>
	where T: Interface
{
	iface: T,
}

impl<T: Interface> Controller<T> 
	where T: Interface
{
	pub fn new(iface: T) -> Controller<T> {
		Controller { iface, }
	}

	fn write_command(&mut self, command: u8) {
		self.iface.write_parameters(command, &[]);
	}

	pub fn write_parameters(&mut self, command: u8, parameters: &[u8]) {
		self.iface.write_parameters(command, parameters);
	}

	fn read_parameters(&mut self, command: u8, parameters: &mut [u8]) {
		self.iface.read_parameters(command, parameters);
	}

	pub fn nop(&mut self) {
		self.write_command(0x00);
	}

	pub fn software_reset(&mut self) {
		self.write_command(0x01);
	}

	pub fn read_display_identification(&mut self) -> DisplayIdentification {
		let mut result = DisplayIdentification::default();
		self.read_parameters(0x04, &mut result.raw);
		result
	}

	pub fn read_display_status(&mut self) -> DisplayStatus {
		let mut result = DisplayStatus::default();
		self.read_parameters(0x09, &mut result.raw);
		result
	}

	pub fn read_display_power_mode(&mut self) -> DisplayPowerMode {
		let mut result = DisplayPowerMode::default();
		self.read_parameters(0x0a, &mut result.raw);
		result
	}

	pub fn read_display_madctl(&mut self) -> MADCtl {
		let mut result = MADCtl::default();
		self.read_parameters(0x0b, &mut result.raw);
		result
	}

	pub fn read_pixel_format(&mut self) -> PixelFormat {
		let mut result = PixelFormat::default();
		self.read_parameters(0x0c, &mut result.raw);
		result
	}

	pub fn read_image_format(&mut self) -> ImageFormat {
		let mut result = ImageFormat::default();
		self.read_parameters(0x0d, &mut result.raw);
		result
	}

	pub fn read_signal_mode(&mut self) -> SignalMode {
		let mut result = SignalMode::default();
		self.read_parameters(0x0e, &mut result.raw);
		result
	}

	pub fn read_self_diagnostic_result(&mut self) -> SelfDiagnosticResult {
		let mut result = SelfDiagnosticResult::default();
		self.read_parameters(0x0f, &mut result.raw);
		result
	}

	pub fn enter_sleep_mode(&mut self) {
		self.write_command(0x10);
	}

	pub fn sleep_out(&mut self) {
		self.write_command(0x11);
	}

	pub fn partial_mode_on(&mut self) {
		self.write_command(0x12);
	}

	pub fn normal_display_mode_on(&mut self) {
		self.write_command(0x13);
	}

	pub fn display_inversion(&mut self, on: bool) {
		let command = if on {
    			0x20
            } else {
    			0x21
    		};
		self.write_command(command);
	}

	pub fn gamma_set(&mut self, gc: u8) {
		self.write_parameters(0x26, &[gc]);
	}

	pub fn display(&mut self, on: bool) {
		let command = if on {
    			0x28
            } else {
    			0x29
    		};
		self.write_command(command);
	}

	pub fn column_address_set(&mut self, sc: u16, ec: u16) {
		self.write_parameters(0x2a, &[
			(sc >> 8) as u8, (sc & 0xff) as u8,
			(ec >> 8) as u8, (ec & 0xff) as u8,
		]);
	}

	pub fn page_address_set(&mut self, sp: u16, ep: u16) {
		self.write_parameters(0x2b, &[
			(sp >> 8) as u8, (sp & 0xff) as u8,
			(ep >> 8) as u8, (ep & 0xff) as u8,
		]);
	}

	pub fn memory_write_start(&mut self) {
		self.write_command(0x2c);
	}

	pub fn color_set(&mut self, data: &[u8; 128]) {
		self.write_parameters(0x2d, data);
	}

	pub fn memory_read_start(&mut self) {
		self.write_command(0x2e);
	}

	pub fn partial_area(&mut self, sr: u16, er: u16) {
		self.write_parameters(0x30, &[
			(sr >> 8) as u8, (sr & 0xff) as u8,
			(er >> 8) as u8, (er & 0xff) as u8,
		]);
	}

	pub fn vertical_scrolling_definition(&mut self, tfa: u16, vsa: u16, bfa: u16) {
		self.write_parameters(0x33, &[
			(tfa >> 8) as u8, (tfa & 0xff) as u8,
			(vsa >> 8) as u8, (vsa & 0xff) as u8,
			(bfa >> 8) as u8, (bfa & 0xff) as u8,
		]);
	}

	pub fn tearing_effect(&mut self, mode: TearingEffect) {
		match mode {
			TearingEffect::VBlankOnly => self.write_parameters(0x35, &[0u8]),
			TearingEffect::HAndVBlank => self.write_parameters(0x35, &[1u8]),
			_                         => self.write_command(0x34),
		};
	}

	pub fn memory_access_control(&mut self, value: MemoryAccessControl) {
		self.write_parameters(0x36, &value.raw);
	}

	pub fn vertical_scrolling_start_address(&mut self, vsp: u16) {
		self.write_parameters(0x37, &[
			(vsp >> 8) as u8, (vsp & 0xff) as u8,
		]);
	}

	pub fn idle_mode(&mut self, on: bool) {
		let command = if on {
    			0x38
            } else {
    			0x39
    		};
		self.write_command(command);
	}

	pub fn pixel_format_set(&mut self, value: PixelFormat) {
		self.write_parameters(0x3a, &value.raw);
	}

	pub fn write_memory_continue(&mut self) {
		self.write_command(0x3c);
	}

	//pub fn write_memory<I>(&mut self, iterable: I)
	//	where I: IntoIterator<Item=u32>
	//{
	//	self.iface.write_memory(iterable);
	//}
    pub fn write_memory(&mut self, data: &[u8]) {
        self.iface.write_memory(data);
    }
    

	pub fn read_memory_continue(&mut self) {
		self.write_command(0x3e);
	}

    //pub fn read_memory(&mut self, data: &mut [u32]) {
	pub fn read_memory(&mut self, data: &mut [u8]) {
		self.iface.read_memory(data);
	}
	
	pub fn set_tear_scanline(&mut self, sts: u16) {
		self.write_parameters(0x44, &[
			(sts >> 8) as u8, (sts & 0xff) as u8,
		]);
	}

	pub fn get_scanline(&mut self) -> u16 {
		let mut result = [0u8; 2];
		self.read_parameters(0x45, &mut result);
		((result[0] as u16) << 8) | result[1] as u16
	}

	pub fn write_display_brightness(&mut self, dbv: u8) {
		self.write_parameters(0x51, &[dbv]);
	}

	pub fn read_display_brightness(&mut self) -> u8 {
		let mut result = [0u8; 1];
		self.read_parameters(0x52, &mut result);
		result[0]
	}

	pub fn write_ctrl_display(&mut self, value: CtrlDisplay) {
		self.write_parameters(0x53, &value.raw);
	}

	pub fn read_ctrl_display(&mut self) -> CtrlDisplay {
		let mut result = CtrlDisplay::default();
		self.read_parameters(0x54, &mut result.raw);
		result
	}

	pub fn write_cabc(&mut self, c: u8) {
		self.write_parameters(0x55, &[c]);
	}

	pub fn read_cabc(&mut self) -> u8 {
		let mut result = [0u8; 1];
		self.read_parameters(0x56, &mut result);
		result[0]
	}

	pub fn write_cabc_minimum_brightness(&mut self, cmb: u8) {
		self.write_parameters(0x5e, &[cmb]);
	}

	pub fn read_cabc_minimum_brightness(&mut self) -> u8 {
		let mut result = [0u8; 1];
		self.read_parameters(0x5f, &mut result);
		result[0]
	}

	pub fn read_id1(&mut self) -> u8 {
		let mut result = [0u8; 1];
		self.read_parameters(0xda, &mut result);
		result[0]
	}

	pub fn read_id2(&mut self) -> u8 {
		let mut result = [0u8; 1];
		self.read_parameters(0xdb, &mut result);
		result[0]
	}

	pub fn read_id3(&mut self) -> u8 {
		let mut result = [0u8; 1];
		self.read_parameters(0xdc, &mut result);
		result[0]
	}

	// TODO: Implement extended command set
}
