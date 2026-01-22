use sysinfo::{
    System,
};

use std::process::{Command};

const SCRAP_TEMPLATE: &str = r#"
@@@@@@@@@::.....:@@@    {hostname}
@: :..... ......:@@@    =========
:  ....:-===++++@@@@    OS - {sysname}
:. .=++++++++===::@@    Kernel - {kernel}
@.  :+===-..... ..:@
@ .. .....:-==+:  .@    Mem - {mem_use}/{mem_total} MB
@@..====+++++++=. .:    Swap - {swap_use}/{swap_total} MB
@@@@++=+==--:.... .:
@@@::..... ......:@@    Scrap is {scrap_status} on your {sysname}
@@@:... ..:@@@@@@@@@
"#;

fn main() {
    let mut system = System::new_all();

    let output = Command::new("which")
        .args(vec!["scrap"])
        .output()
        .expect("Failed to check scrap installation");

    system.refresh_all();

    let hostname = System::host_name().unwrap();
    let sysname = System::name().unwrap();
    let kernel = System::kernel_long_version();
    let mem_use = system.used_memory() / (1024 * 1024);
    let mem_total = system.total_memory() / (1024 * 1024);
    let swap_use = system.used_swap() / (1024 * 1024);
    let swap_total = system.total_swap() / (1024 * 1024);
    let scrap_status = if output.status.code().unwrap() != 0 { "not installed".to_string() } else { "installed".to_string() };

    let formatted = strfmt::strfmt!(SCRAP_TEMPLATE, hostname, sysname.clone(), kernel, mem_use, mem_total, swap_use, swap_total, scrap_status, sysname);

    println!("{}", formatted.unwrap());
}
