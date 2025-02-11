/**
 * fastfetch - ff; for Windows by .ninju.
 * @author .ninju.
 * @version 0.0.0
 */

/*
Console Style Modifiers:
1   - Bold
2   - Dim
3   - Italic
4   - Underline
5   - Blink
7   - Reverse
8   - Hidden
30  - Black
31  - Red
32  - Green
33  - Yellow
34  - Blue
35  - Magenta
36  - Cyan
37  - White
40  - Black background
41  - Red background
42  - Green background
43  - Yellow background
44  - Blue background
45  - Magenta background
46  - Cyan background
47  - White background
90  - Bright Black (Gray)
91  - Bright Red
92  - Bright Green
93  - Bright Yellow
94  - Bright Blue
95  - Bright Magenta
96  - Bright Cyan
97  - Bright White
100 - Bright Black background
101 - Bright Red background
102 - Bright Green background
103 - Bright Yellow background
104 - Bright Blue background
105 - Bright Magenta background
106 - Bright Cyan background
107 - Bright White background
HEX - \x1b[38;2;R;G;Bm
*/

use battery;
use crossterm;
use winreg::RegKey;
use winreg::enums::*;
use unicode_width::UnicodeWidthStr;
use chrono::Duration;
use num_cpus;
use sysinfo;

fn get_pc_name() -> String {
    std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string())
}
fn get_username() -> String {
    std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string())
}

fn get_battery_info() -> String {
    let manager = battery::Manager::new().unwrap();
    let mut info = String::new();

    for battery in manager.batteries().unwrap() {
        let battery = battery.unwrap();
        info.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between(
                "Percentage:",
                format!(
                    "{:.1}%{}",
                    battery.state_of_charge().value * 100.0,
                    if battery.state() == battery::State::Charging { " " } else { "" }
                )
                .as_str(),
                50
            )
        ));
        info.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between(
                "State:",
                battery.state().to_string().as_str(),
                50
            )
        ));
        info.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between(
                "Energy:",
                &format!("{:.0}KWh", battery.energy().value as f64 / 1000.0),
                50
            )
        ));
        info.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between(
                "Energy Full:",
                &format!("{:.0}KWh", battery.energy_full().value as f64 / 1000.0),
                50
            )
        ));
        info.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between(
                "Voltage:",
                &format!("{:.2}V", battery.voltage().value),
                50
            )
        ));
        if let Some(time_to_full) = battery.time_to_full() {
            info.push_str(&format!(
                "\x1b[38;2;255;207;239m{}\x1b[0m\n",
                flex_between(
                    "Time to Full:",
                    &format!("{:.1} hours", time_to_full.value / 3600.0),
                    50
                )
            ));
        }
        if let Some(time_to_empty) = battery.time_to_empty() {
            let duration = Duration::seconds(time_to_empty.value as i64);
            info.push_str(&format!(
                "\x1b[38;2;255;207;239m{}\x1b[0m\n",
                flex_between(
                    "Time to Empty:",
                    &format!("{:02}:{:02}", duration.num_hours(), duration.num_minutes() % 60),
                    50
                )
            ));
        }
    }
    info.push_str("\x1b[0m\n");
    info
}

fn get_processor_info() -> String {
    let mut str = String::new();
    let mut system = sysinfo::System::new_all();
    system.refresh_all();
    if let Some(cpu) = system.cpus().first() {
        println!("{:?}", cpu);
        str.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between("Model:", &cpu.brand().trim(), 50)
        ));
        str.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between("frequency:", &format!("{:.2}GHz", cpu.frequency() as f64 / 1000.0), 50)
        ));
        str.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between("Vendor:", &cpu.vendor_id().trim(), 50)
        ));
        str.push_str(&format!(
            "\x1b[38;2;255;207;239m{}\x1b[0m\n",
            flex_between("Usage:", &(cpu.cpu_usage().to_string() + "%"), 50)
        ));
    }
    let num_cores = num_cpus::get();
    str.push_str(&format!(
        "\x1b[38;2;255;207;239m{}\x1b[0m\n",
        flex_between("Cores:", &num_cores.to_string(), 50)
    ));

    str
}

fn get_console_width() -> u16 {
    let (width, _) = crossterm::terminal::size().unwrap();
    width
}
fn get_os_version() -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(current_version) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        let product_name: String = current_version.get_value("ProductName").unwrap_or_else(|_| "Unknown".into());
        let display_version: String = current_version.get_value("DisplayVersion").unwrap_or_else(|_| "Unknown".into());
        return format!("{} {}", product_name, display_version);
    }
    "Unknown".to_string()
}
fn format_header() -> String {
    let userstr: String = format!("{} {}@{}", "", get_username(), get_pc_name());
/*     str.push_str(format!("\x1b[1;38;2;255;49;187m{}\x1b[0m", userstr).as_str());
    let current_len: u16 = UnicodeWidthStr::width(userstr.as_str()) as u16; */
    let os_version_str = get_os_version();
    format!("\x1b[1;38;2;255;49;187m{}", flex_between(userstr.as_str(), os_version_str.as_str(), 50))
    
}
fn subtitle(string: &str) -> String {
    let width = get_console_width();
    let mut str = String::new();
    let start = (width - UnicodeWidthStr::width(string) as u16) / 2;
    for i in 0..width {
        if i >= start && i < start + UnicodeWidthStr::width(string) as u16 {
            str.push_str(string.chars().nth((i - start) as usize).unwrap().to_string().as_str());
        } else {
            str.push_str(" ");
        }
    }
    str
}

fn gradient_delim(start_hex: u32, end_hex: u32, width_in_percent: u16) -> String {
    let total_width = get_console_width();
    let length = (total_width as f32 * width_in_percent as f32 / 100.0).round() as u16;
    let start_r = ((start_hex >> 16) & 0xFF) as f32;
    let start_g = ((start_hex >> 8) & 0xFF) as f32;
    let start_b = (start_hex & 0xFF) as f32;
    let end_r = ((end_hex >> 16) & 0xFF) as f32;
    let end_g = ((end_hex >> 8) & 0xFF) as f32;
    let end_b = (end_hex & 0xFF) as f32;
    let padding = (total_width - length) / 2;
    let mut str = String::new();
    for i in 0..total_width {
        if i < padding || i >= padding + length {
            str.push_str(" ");
        } else {
            let t = (i - padding) as f32 / (length - 1) as f32;
            let r = (start_r + (end_r - start_r) * t).round() as u32;
            let g = (start_g + (end_g - start_g) * t).round() as u32;
            let b = (start_b + (end_b - start_b) * t).round() as u32;
            str.push_str(&format!("\x1b[38;2;{};{};{}m▀\x1b[0m", r, g, b));
        }
    }
    str
}

fn flex_between(str1: &str, str2: &str, width_in_percent: u16 /* 1 - 100 */) -> String {
    if width_in_percent == 0 {
        return "".to_string();
    }
    let console_width = get_console_width();
    let side_padding = (console_width as f32 * (100.0 - width_in_percent as f32) / 100.0 / 2.0) as u16;
    let mut str = String::new();
    for i in 0..console_width {
        if i < side_padding {
            str.push_str(" ");
        } else if i < side_padding + UnicodeWidthStr::width(str1) as u16 {
            str.push_str(str1.chars().nth((i - side_padding) as usize).unwrap().to_string().as_str());
        } else if i < console_width - side_padding - UnicodeWidthStr::width(str2) as u16 {
            str.push_str(" ");
        } else if i < console_width - side_padding {
            str.push_str(str2.chars().nth((i - (console_width - side_padding - UnicodeWidthStr::width(str2) as u16)) as usize).unwrap().to_string().as_str());
        } else {
            str.push_str(" ");
        }
    }
    str
}

fn main() {
    let mut str = String::new();
    // get pc
    str.push_str(format!("{}\n", format_header()).as_str());
    str.push_str((gradient_delim(0xffffff, 0xff31bb, 50) + "\n").as_str());
    str.push_str(format!("\x1b[1;38;2;255;153;221m{}\x1b[0m\n", subtitle("Battery")).as_str());
    str.push_str(&get_battery_info());
    str.push_str("\n");
    str.push_str(format!("\x1b[1;38;2;255;153;221m{}\x1b[0m\n", subtitle(format!("{} {}", "\u{f4bc}", "Processor").as_str())).as_str());
    str.push_str(&get_processor_info());
    print!("{}", str);
}