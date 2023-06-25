//! # Neotron QEMU BIOS
//!
//! This is the BIOS for Neotron OS on QEMU.

#![no_std]
#![no_main]

mod mutex;

use core::fmt::Write;

use cortex_m_rt::entry;
use neotron_common_bios as common;

extern "C" {
    static mut _flash_os_start: u32;
    static mut _flash_os_len: u32;
    static mut _ram_os_start: u32;
    static mut _ram_os_len: u32;
}

/// Where the OS can put the text characters
static mut VRAM: [(u8, u8); 80 * 50] = [(0, 0); 80 * 50];

/// The clock speed of the peripheral subsystem on an SSE-300 SoC an on MPS3 board
const PERIPHERAL_CLOCK: u32 = 25_000_000;

/// BIOS Version
static BIOS_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The API we provide to the OS
static API_CALLS: common::Api = common::Api {
    api_version_get,
    bios_version_get,
    serial_get_info,
    serial_configure,
    serial_write,
    serial_read,
    time_clock_get,
    time_clock_set,
    time_ticks_get,
    time_ticks_per_second,
    configuration_get,
    configuration_set,
    video_is_valid_mode,
    video_mode_needs_vram,
    video_set_mode,
    video_get_mode,
    video_get_framebuffer,
    video_set_framebuffer,
    video_wait_for_line,
    video_get_palette,
    video_set_palette,
    video_set_whole_palette,
    memory_get_region,
    hid_get_event,
    hid_set_leds,
    i2c_bus_get_info,
    i2c_write_read,
    audio_mixer_channel_get_info,
    audio_mixer_channel_set_level,
    audio_output_set_config,
    audio_output_get_config,
    audio_output_data,
    audio_output_get_space,
    audio_input_set_config,
    audio_input_get_config,
    audio_input_data,
    audio_input_get_count,
    bus_select,
    bus_get_info,
    bus_write_read,
    bus_exchange,
    bus_interrupt_status,
    block_dev_get_info,
    block_dev_eject,
    block_write,
    block_read,
    block_verify,
    power_idle,
};

/// A driver for CMSDK Uart
struct Uart<const ADDR: usize>();

impl<const ADDR: usize> Uart<ADDR> {
    const STATUS_TX_FULL: u32 = 1 << 0;
    const STATUS_RX_NON_EMPTY: u32 = 1 << 1;

    /// Turn on TX and RX
    fn enable(&mut self, baudrate: u32, system_clock: u32) {
        let divider = system_clock / baudrate;
        self.set_bauddiv(divider);
        self.set_control(0b0000_0011);
    }

    /// Write a byte (blocking if there's no space)
    fn write(&mut self, byte: u8) {
        // Check the Buffer Full bit
        while (self.get_status() & Self::STATUS_TX_FULL) != 0 {}
        self.set_data(byte as u32);
    }

    /// Try and read a byte
    fn read(&mut self) -> Option<u8> {
        if (self.get_status() & Self::STATUS_RX_NON_EMPTY) != 0 {
            Some(self.get_data() as u8)
        } else {
            None
        }
    }

    /// Read the data register
    fn get_data(&mut self) -> u32 {
        let ptr = ADDR as *mut u32;
        unsafe { ptr.read_volatile() }
    }

    /// Write the data register
    fn set_data(&mut self, data: u32) {
        let ptr = ADDR as *mut u32;
        unsafe { ptr.write_volatile(data) }
    }

    /// Read the status register
    fn get_status(&self) -> u32 {
        let ptr = (ADDR + 4) as *mut u32;
        unsafe { ptr.read_volatile() }
    }

    /// Set the control register
    fn set_control(&mut self, data: u32) {
        let ptr = (ADDR + 8) as *mut u32;
        unsafe { ptr.write_volatile(data) }
    }

    /// Set the baud rate divider register
    fn set_bauddiv(&mut self, data: u32) {
        let ptr = (ADDR + 16) as *mut u32;
        unsafe { ptr.write_volatile(data) }
    }
}

impl<const N: usize> core::fmt::Write for Uart<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            self.write(b);
        }
        Ok(())
    }
}

/// Describes the hardware in the system
struct Hardware {
    _cp: cortex_m::Peripherals,
    uart0: Uart<0x5930_3000>,
}

static HARDWARE: mutex::NeoMutex<Option<Hardware>> = mutex::NeoMutex::new(None);

/// Entry point for the BIOS. This is called by the startup code.
#[entry]
fn bios_main() -> ! {
    // Set up the hardware
    let mut h = hardware_setup();

    // Print the BIOS version
    write!(h.uart0, "Neotron QEMU BIOS {}\r\n", BIOS_VERSION).unwrap();

    *HARDWARE.lock() = Some(h);

    neotron_os::os_main(&API_CALLS)
}

/// Configure the hardware
fn hardware_setup() -> Hardware {
    let mut uart0 = Uart();
    uart0.enable(115200, PERIPHERAL_CLOCK);
    Hardware {
        _cp: cortex_m::Peripherals::take().expect("Couldn't get hardware"),
        uart0,
    }
}

/// Returns the version number of the BIOS API.
pub extern "C" fn api_version_get() -> common::Version {
    common::API_VERSION
}

/// Returns a pointer to a static string slice containing the BIOS Version.
///
/// This string contains the version number and build string of the BIOS.
/// For C compatibility this string is null-terminated and guaranteed to
/// only contain ASCII characters (bytes with a value 127 or lower). We
/// also pass the length (excluding the null) to make it easy to construct
/// a Rust string. It is unspecified as to whether the string is located
/// in Flash ROM or RAM (but it's likely to be Flash ROM).
pub extern "C" fn bios_version_get() -> common::ApiString<'static> {
    common::ApiString::new(BIOS_VERSION)
}

/// Get information about the Serial ports in the system.
///
/// Serial ports are ordered octet-oriented pipes. You can push octets
/// into them using a 'write' call, and pull bytes out of them using a
/// 'read' call. They have options which allow them to be configured at
/// different speeds, or with different transmission settings (parity
/// bits, stop bits, etc) - you set these with a call to
/// `SerialConfigure`. They may physically be a MIDI interface, an RS-232
/// port or a USB-Serial port. There is no sense of 'open' or 'close' -
/// that is an Operating System level design feature. These APIs just
/// reflect the raw hardware, in a similar manner to the registers exposed
/// by a memory-mapped UART peripheral.
pub extern "C" fn serial_get_info(device: u8) -> common::Option<common::serial::DeviceInfo> {
    if device == 0 {
        common::Option::Some(common::serial::DeviceInfo {
            name: common::ApiString::new("ser0"),
            device_type: common::serial::DeviceType::TtlUart,
        })
    } else {
        common::Option::None
    }
}

/// Set the options for a given serial device. An error is returned if the
/// options are invalid for that serial device.
pub extern "C" fn serial_configure(
    device: u8,
    config: common::serial::Config,
) -> common::Result<()> {
    if device == 0 {
        let mut hw = HARDWARE.lock();
        let hw = hw.as_mut().unwrap();
        // Ignore all the settings and just turn the thing on
        hw.uart0.enable(config.data_rate_bps, PERIPHERAL_CLOCK);
    }
    common::Result::Ok(())
}

/// Write bytes to a serial port. There is no sense of 'opening' or
/// 'closing' the device - serial devices are always open. If the return
/// value is `Ok(n)`, the value `n` may be less than the size of the given
/// buffer. If so, that means not all of the data could be transmitted -
/// only the first `n` bytes were.
pub extern "C" fn serial_write(
    device: u8,
    data: common::ApiByteSlice,
    _timeout: common::Option<common::Timeout>,
) -> common::Result<usize> {
    if device == 0 {
        let mut hw = HARDWARE.lock();
        let hw = hw.as_mut().unwrap();
        let bytes = data.as_slice();
        for b in bytes {
            if *b == b'\n' {
                hw.uart0.write(b'\r');
            }
            hw.uart0.write(*b);
        }
        common::Result::Ok(bytes.len())
    } else {
        common::Result::Err(common::Error::InvalidDevice)
    }
}

/// Read bytes from a serial port. There is no sense of 'opening' or
/// 'closing' the device - serial devices are always open. If the return value
///  is `Ok(n)`, the value `n` may be less than the size of the given buffer.
///  If so, that means not all of the data could be received - only the
///  first `n` bytes were filled in.
pub extern "C" fn serial_read(
    device: u8,
    mut data: common::ApiBuffer,
    _timeout: common::Option<common::Timeout>,
) -> common::Result<usize> {
    if device == 0 {
        let mut hw = HARDWARE.lock();
        let hw = hw.as_mut().unwrap();
        let Some(bytes) = data.as_mut_slice() else {
            return common::Result::Err(common::Error::UnsupportedConfiguration(0));
        };
        let mut count = 0;
        for b in bytes {
            if let Some(read) = hw.uart0.read() {
                *b = read;
                count += 1;
            } else {
                break;
            }
        }
        common::Result::Ok(count)
    } else {
        common::Result::Err(common::Error::InvalidDevice)
    }
}

/// Get the current wall time.
///
/// The Neotron BIOS does not understand time zones, leap-seconds or the
/// Gregorian calendar. It simply stores time as an incrementing number of
/// seconds since some epoch, and the number of milliseconds since that second
/// began. A day is assumed to be exactly 86,400 seconds long. This is a lot
/// like POSIX time, except we have a different epoch
/// - the Neotron epoch is 2000-01-01T00:00:00Z. It is highly recommend that you
/// store UTC in the BIOS and use the OS to handle time-zones.
///
/// If the BIOS does not have a battery-backed clock, or if that battery has
/// failed to keep time, the system starts up assuming it is the epoch.
pub extern "C" fn time_clock_get() -> common::Time {
    common::Time { secs: 0, nsecs: 0 }
}

/// Set the current wall time.
///
/// See `time_get` for a description of now the Neotron BIOS should handle
/// time.
///
/// You only need to call this whenever you get a new sense of the current
/// time (e.g. the user has updated the current time, or if you get a GPS
/// fix). The BIOS should push the time out to the battery-backed Real
/// Time Clock, if it has one.
pub extern "C" fn time_clock_set(_time: common::Time) {}

/// Get the configuration data block.
///
/// Configuration data is, to the BIOS, just a block of bytes of a given length.
/// How it stores them is up to the BIOS - it could be EEPROM, or battery-backed
/// SRAM.
///
/// However, in this BIOS we are linked to the OS so we can cheat and encode the
/// bytes with postcard directly.
pub extern "C" fn configuration_get(mut buffer: common::ApiBuffer) -> common::Result<usize> {
    let mut config = neotron_os::OsConfig::default();
    config.set_serial_console_on(115_200);
    config.set_vga_console(false);

    let Some(buffer) = buffer.as_mut_slice() else {
        return common::Result::Err(common::Error::UnsupportedConfiguration(0));
    };
    match postcard::to_slice(&config, buffer) {
        Ok(slice) => common::Result::Ok(slice.len()),
        Err(_e) => common::Result::Err(common::Error::UnsupportedConfiguration(0)),
    }
}

/// Set the configuration data block.
///
/// See `configuration_get`.
pub extern "C" fn configuration_set(_buffer: common::ApiByteSlice) -> common::Result<()> {
    common::Result::Ok(())
}

/// Does this Neotron BIOS support this video mode?
pub extern "C" fn video_is_valid_mode(_mode: common::video::Mode) -> bool {
    false
}

/// Switch to a new video mode.
///
/// The contents of the screen are undefined after a call to this function.
///
/// If the BIOS does not have enough reserved RAM (or dedicated VRAM) to
/// support this mode, the change will succeed but a subsequent call to
/// `video_get_framebuffer` will return `null`. You must then supply a
/// pointer to a block of size `Mode::frame_size_bytes()` to
/// `video_set_framebuffer` before any video will appear.
pub extern "C" fn video_set_mode(_mode: common::video::Mode) -> common::Result<()> {
    common::Result::Err(common::Error::UnsupportedConfiguration(0))
}

/// Returns the video mode the BIOS is currently in.
///
/// The OS should call this function immediately after start-up and note
/// the value - this is the `default` video mode which can always be
/// serviced without supplying extra RAM.
pub extern "C" fn video_get_mode() -> common::video::Mode {
    unsafe { common::video::Mode::from_u8(0) }
}

/// Get the framebuffer address.
///
/// We can write through this address to the video framebuffer. The
/// meaning of the data we write, and the size of the region we are
/// allowed to write to, is a function of the current video mode (see
/// `video_get_mode`).
///
/// This function will return `null` if the BIOS isn't able to support the
/// current video mode from its memory reserves. If that happens, you will
/// need to use some OS RAM or Application RAM and provide that as a
/// framebuffer to `video_set_framebuffer`. The BIOS will always be able
/// to provide the 'basic' text buffer experience from reserves, so this
/// function will never return `null` on start-up.
pub extern "C" fn video_get_framebuffer() -> *mut u8 {
    unsafe { VRAM.as_mut_ptr() as *mut u8 }
}

/// Set the framebuffer address.
///
/// Tell the BIOS where it should start fetching pixel or textual data from
/// (depending on the current video mode).
///
/// This value is forgotten after a video mode change and must be re-supplied.
///
/// # Safety
///
/// The pointer must point to enough video memory to handle the current video
/// mode, and any future video mode you set.
pub unsafe extern "C" fn video_set_framebuffer(_buffer: *const u8) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

/// Find out whether the given video mode needs more VRAM than we currently have.
///
/// The answer is no for any currently supported video mode (which is just the four text modes right now).
pub extern "C" fn video_mode_needs_vram(_mode: common::video::Mode) -> bool {
    false
}

/// Find out how large a given region of memory is.
///
/// The first region is the 'main application region' and is defined to always
/// start at address `0x2000_0000` on a standard Cortex-M system. This
/// application region stops just before the BIOS reserved memory, at the top of
/// the internal SRAM. The OS will have been linked to use the first 1 KiB of
/// this region.
///
/// Other regions may be located at other addresses (e.g. external DRAM or
/// PSRAM).
///
/// The OS will always load non-relocatable applications into the bottom of
/// Region 0. It can allocate OS specific structures from any other Region (if
/// any), or from the top of Region 0 (although this reduces the maximum
/// application space available). The OS will prefer lower numbered regions
/// (other than Region 0), so faster memory should be listed first.
///
/// If the region number given is invalid, the function returns `(null, 0)`.
pub extern "C" fn memory_get_region(region: u8) -> common::Option<common::MemoryRegion> {
    match region {
        0 => {
            // Application Region
            common::Option::Some(common::MemoryRegion {
                start: unsafe { &mut _ram_os_start as *mut u32 } as *mut u8,
                length: unsafe { &mut _ram_os_len as *const u32 } as usize,
                kind: common::MemoryKind::Ram,
            })
        }
        _ => common::Option::None,
    }
}

/// This function doesn't block. It will return `Ok(None)` if there is no event
/// ready.
///
/// The Pico BIOS gets PS/2 scan-codes (in PS/2 Scan Code Set 2 format) from the
/// BMC. The BMC receives them from the PS/2 keyboard (as 11-bit words with
/// stop, stop and parity bits) and buffers them (as raw 8-bit values with the
/// start/stop/parity bits removed). These scan-codes are converted into
/// human-readable labels here in this function. The labels are applied as if
/// you had a US-English keyboard. If you do not have a US-English keyboard, the
/// labels, will be incorrect, but that doesn't matter. It is the OS's job to
/// convert those labels (along with the key up or key down event) into Unicode
/// characters, which is where the country-specific keyboard mapping comes in.
///
/// This is a similar model used to that in the IBM PC. Your PC's BIOS cares not
/// for which country you are; that was MS-DOS's job.
///
/// The reason we don't just pass keyboard scan-codes in Scan Code Set 2 (the
/// power-up default for almost every IBM PS/2 compatible keyboard) is that in
/// the future your BIOS key get the keyboard input from another source. If it
/// came from a USB Keyboard, you would have USB HID Scan Codes. If it came from
/// an SDL2 window under Linux/Windows/macOS, you would have SDL2 specific key
/// codes. So the BIOS must convert this wide and varied set of HID inputs into
/// a single KeyCode enum. Plus, Scan Code Set 2 is a pain, because most of the
/// 'extended' keys they added on the IBM PC/AT actually generate two bytes, not
/// one. It's much nicer when your Scan Codes always have one byte per key.
pub extern "C" fn hid_get_event() -> common::Result<common::Option<common::hid::HidEvent>> {
    common::Result::Ok(common::Option::None)
}

/// Control the keyboard LEDs.
pub extern "C" fn hid_set_leds(_leds: common::hid::KeyboardLeds) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

/// Wait for the next occurence of the specified video scan-line.
///
/// In general we must assume that the video memory is read top-to-bottom
/// as the picture is being drawn on the monitor (e.g. via a VGA video
/// signal). If you modify video memory during this *drawing period*
/// there is a risk that the image on the monitor (however briefly) may
/// contain some parts from before the modification and some parts from
/// after. This can given rise to the *tearing effect* where it looks
/// like the screen has been torn (or ripped) across because there is a
/// discontinuity part-way through the image.
///
/// This function busy-waits until the video drawing has reached a
/// specified scan-line on the video frame.
///
/// There is no error code here. If the line you ask for is beyond the
/// number of visible scan-lines in the current video mode, it waits util
/// the last visible scan-line is complete.
///
/// If you wait for the last visible line until drawing, you stand the
/// best chance of your pixels operations on the video RAM being
/// completed before scan-lines start being sent to the monitor for the
/// next frame.
///
/// You can also use this for a crude `16.7 ms` delay but note that
/// some video modes run at `70 Hz` and so this would then give you a
/// `14.3ms` second delay.
pub extern "C" fn video_wait_for_line(_line: u16) {}

/// Read the RGB palette.
extern "C" fn video_get_palette(_index: u8) -> common::Option<common::video::RGBColour> {
    common::Option::None
}

/// Update the RGB palette.
extern "C" fn video_set_palette(_index: u8, _rgb: common::video::RGBColour) {}

/// Update all the RGB palette
unsafe extern "C" fn video_set_whole_palette(
    _palette: *const common::video::RGBColour,
    _length: usize,
) {
}

extern "C" fn i2c_bus_get_info(_i2c_bus: u8) -> common::Option<common::i2c::BusInfo> {
    common::Option::None
}

extern "C" fn i2c_write_read(
    _i2c_bus: u8,
    _i2c_device_address: u8,
    _tx: common::ApiByteSlice,
    _tx2: common::ApiByteSlice,
    _rx: common::ApiBuffer,
) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn audio_mixer_channel_get_info(
    _audio_mixer_id: u8,
) -> common::Option<common::audio::MixerChannelInfo> {
    common::Option::None
}

extern "C" fn audio_mixer_channel_set_level(_audio_mixer_id: u8, _level: u8) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn audio_output_set_config(_config: common::audio::Config) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn audio_output_get_config() -> common::Result<common::audio::Config> {
    common::Result::Err(common::Error::Unimplemented)
}

unsafe extern "C" fn audio_output_data(_samples: common::ApiByteSlice) -> common::Result<usize> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn audio_output_get_space() -> common::Result<usize> {
    common::Result::Ok(0)
}

extern "C" fn audio_input_set_config(_config: common::audio::Config) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn audio_input_get_config() -> common::Result<common::audio::Config> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn audio_input_data(_samples: common::ApiBuffer) -> common::Result<usize> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn audio_input_get_count() -> common::Result<usize> {
    common::Result::Ok(0)
}

extern "C" fn bus_select(_periperal_id: common::Option<u8>) {
    // Do nothing
}

extern "C" fn bus_get_info(_periperal_id: u8) -> common::Option<common::bus::PeripheralInfo> {
    common::Option::None
}

extern "C" fn bus_write_read(
    _tx: common::ApiByteSlice,
    _tx2: common::ApiByteSlice,
    _rx: common::ApiBuffer,
) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn bus_exchange(_buffer: common::ApiBuffer) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn bus_interrupt_status() -> u32 {
    0
}

/// Get information about the Block Devices in the system.
///
/// Block Devices are also known as *disk drives*. They can be read from
/// (and often written to) but only in units called *blocks* or *sectors*.
///
/// The BIOS should enumerate removable devices first, followed by fixed
/// devices.
///
/// The set of devices is not expected to change at run-time - removal of
/// media is indicated with a boolean field in the
/// `block_dev::DeviceInfo` structure.
pub extern "C" fn block_dev_get_info(_device: u8) -> common::Option<common::block_dev::DeviceInfo> {
    common::Option::None
}

/// Write one or more sectors to a block device.
///
/// The function will block until all data is written. The array pointed
/// to by `data` must be `num_blocks * block_size` in length, where
/// `block_size` is given by `block_dev_get_info`.
///
/// There are no requirements on the alignment of `data` but if it is
/// aligned, the BIOS may be able to use a higher-performance code path.
pub extern "C" fn block_write(
    _device: u8,
    _block: common::block_dev::BlockIdx,
    _num_blocks: u8,
    _data: common::ApiByteSlice,
) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

/// Read one or more sectors to a block device.
///
/// The function will block until all data is read. The array pointed
/// to by `data` must be `num_blocks * block_size` in length, where
/// `block_size` is given by `block_dev_get_info`.
///
/// There are no requirements on the alignment of `data` but if it is
/// aligned, the BIOS may be able to use a higher-performance code path.
pub extern "C" fn block_read(
    _device: u8,
    _block: common::block_dev::BlockIdx,
    _num_blocks: u8,
    _data: common::ApiBuffer,
) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

/// Verify one or more sectors on a block device (that is read them and
/// check they match the given data).
///
/// The function will block until all data is verified. The array pointed
/// to by `data` must be `num_blocks * block_size` in length, where
/// `block_size` is given by `block_dev_get_info`.
///
/// There are no requirements on the alignment of `data` but if it is
/// aligned, the BIOS may be able to use a higher-performance code path.
pub extern "C" fn block_verify(
    _device: u8,
    _block: common::block_dev::BlockIdx,
    _num_blocks: u8,
    _data: common::ApiByteSlice,
) -> common::Result<()> {
    common::Result::Err(common::Error::Unimplemented)
}

extern "C" fn block_dev_eject(_dev_id: u8) -> common::Result<()> {
    common::Result::Ok(())
}

/// Sleep the CPU until the next interrupt.
extern "C" fn power_idle() {}

extern "C" fn time_ticks_get() -> common::Ticks {
    common::Ticks(0)
}

/// We have a 1 MHz timer
extern "C" fn time_ticks_per_second() -> common::Ticks {
    common::Ticks(1_000_000)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut uart0: Uart<0x5930_3000> = Uart();
    let _ = write!(uart0, "PANIC!\r\n{:#?}\r\n", info);
    loop {
        cortex_m::asm::wfi();
    }
}

// End of file
