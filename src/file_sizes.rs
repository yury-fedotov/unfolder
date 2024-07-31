const THOUSAND: usize = 1000;
const MEGA_THOUSAND_EXPONENT: u32 = 2;
const GIGA_THOUSAND_EXPONENT: u32 = 3;
pub const KILOBYTE: usize = THOUSAND;
pub const MEGABYTE: usize = THOUSAND.pow(MEGA_THOUSAND_EXPONENT);
pub const GIGABYTE: usize = THOUSAND.pow(GIGA_THOUSAND_EXPONENT);

/// Converts bytes into a human-readable format (bytes, KB, MB, GB)
pub fn format_size(size: usize) -> String {
    if size >= GIGABYTE {
        format!("{:.2} GB", size / GIGABYTE)
    } else if size >= MEGABYTE {
        format!("{:.2} MB", size / MEGABYTE)
    } else if size >= KILOBYTE {
        format!("{:.2} KB", size / KILOBYTE)
    } else {
        format!("{} bytes", size)
    }
}
