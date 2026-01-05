use sysinfo::{
    System,
};

use std::collections::HashMap;
use std::process::{Command};

const SCRAP_TEMPLATE: &str = r#"
@@@@@@@@@::.....:@@@    {hostname}
@: :..... ......:@@@    =========
:  ....:-===++++@@@@    OS - {osname}
:. .=++++++++===::@@    Kernel - {kernelinfo}
@.  :+===-..... ..:@
@ .. .....:-==+:  .@    Mem - {mem_used}/{mem_total} MB
@@..====+++++++=. .:    Swap - {swap_used}/{swap_total} MB
@@@@++=+==--:.... .:
@@@::..... ......:@@    Scrap is {status} on your {osname}
@@@:... ..:@@@@@@@@@
"#;

fn main() {
    let mut info = HashMap::new();
    let mut system = System::new_all();

    let output = Command::new("which")
        .args(vec!["scrap"])
        .output()
        .expect("Failed to check scrap installation");

    system.refresh_all();

    info.insert("hostname".to_string(), System::host_name().unwrap());
    info.insert("osname".to_string(), System::name().unwrap());
    info.insert("kernelinfo".to_string(), System::kernel_long_version());
    info.insert("mem_used".to_string(), (system.used_memory() / (1024 * 1024)) .to_string());
    info.insert("mem_total".to_string(), (system.total_memory() / (1024 * 1024)).to_string());
    info.insert("swap_used".to_string(), (system.used_swap() / (1024 * 1024)).to_string());
    info.insert("swap_total".to_string(), (system.total_swap() / (1024 * 1024)).to_string());
    info.insert("status".to_string(), if output.status.code().unwrap() != 0 { "not installed".to_string() } else { "installed".to_string() });

    let formatted = strfmt::strfmt(&SCRAP_TEMPLATE.to_string(), &info);

    println!("{}", formatted.unwrap());
}
