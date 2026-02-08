use sysinfo::System;
use std::process::{Command};
use users::get_current_username;

fn main() {
    let mut system = System::new_all();

    let output = Command::new("which")
        .args(vec!["scrap"])
        .output()
        .expect("Failed to check scrap installation");

    system.refresh_all();

    let username: String = if let Some(t) = get_current_username() { t.into_string().unwrap() } else { String::from("") };
    let hostname = System::host_name().unwrap();
    let sysname = System::name().unwrap();
    let kernel = System::kernel_long_version();
    let mem_use = system.used_memory() / (1024 * 1024);
    let mem_total = system.total_memory() / (1024 * 1024);
    let swap_use = system.used_swap() / (1024 * 1024);
    let swap_total = system.total_swap() / (1024 * 1024);
    let scrap_status = if output.status.code().unwrap() != 0 { "not installed".to_string() } else { "installed".to_string() };

    println!("@@@@@@@@@::.....:@@@    {username}@{hostname}");
    println!("@: :..... ......:@@@    {}", vec!["="; format!("{username}@{hostname}").len()].join(""));
    println!(":  ....:-===++++@@@@    OS - {sysname}");
    println!(":. .=++++++++===::@@    Kernel - {kernel}");
    println!("@.  :+===-..... ..:@");
    println!("@ .. .....:-==+:  .@    Mem - {mem_use}/{mem_total} MB");
    println!("@@..====+++++++=. .:    Swap - {swap_use}/{swap_total} MB");
    println!("@@@@++=+==--:.... .:");
    println!("@@@::..... ......:@@    Scrap is {scrap_status} on your {sysname}");
    println!("@@@:... ..:@@@@@@@@@");
}
