use std::{process::Command, path::{PathBuf, Path}};
use chrono::format::format;
use regex::Regex;
use serde::{Deserialize, Serialize, de};
use sysinfo::{DiskExt, System, SystemExt, RefreshKind};

use crate::sizeunit;

#[derive(serde::Serialize, Debug)]
pub struct DriveInformation {
    pub name: String,
    pub mount_point: String,
    pub total: String,
    pub free: String,
    pub is_removable: bool,
    pub disk_type: String,
    pub file_system: String,
}

#[derive(serde::Serialize,Debug)]
pub struct Drives {
    pub array_of_drives: Vec<DriveInformation>,
}

// #[tauri::command]

fn get_lsblk_output() -> Result<Vec<u8>,()> {
    let mut command = Command::new("/bin/lsblk");
    command.args([
        // Print size in bytes
        "--bytes",
        // Select the fields to output
        "--output",
        "NAME,FSTYPE,FSVER,LABEL,UUID,FSAVAIL,FSUSED,MOUNTPOINT,HOTPLUG,SIZE,VENDOR,MODEL,RM,STATE,TYPE,KNAME",
        // Format output as JSON
        "--json",
        // Print full device paths
        "--paths",
        // Exclude some devices by major number. See
        // https://www.kernel.org/doc/Documentation/admin-guide/devices.txt
        // for a list of major numbers.
        //
        // - Exclude floppy drives (2), as they are slow.
        // - Exclude scsi cdrom drives (11), as they are slow.
        // - Exclude zram (253), not a valid install target.
        "--exclude",
        "2,11,253",
    ]);
    
    // let output = String::from_utf8(get_command_output(command).unwrap()).unwrap();
    // Ok(output)
    get_command_output(command)
}
fn get_command_output(mut command: Command) -> Result<Vec<u8>,()> {
    // info!("running command: {:?}", command);
    let output = match command.output() {
        Ok(output) => output,
        Err(err) => {
            return Err(())
            // bail!("Failed to execute command: {err}");
        }
    };

    if !output.status.success() {
        // bail!("Failed to execute command");
    }
    Ok(output.stdout)
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct LsBlkDeviceWithChildren {
    #[serde(flatten)]
    details: LsBlkDevice,


    /// Child devices.
    #[serde(default)]
    children: Vec<LsBlkDeviceWithChildren>,
}
#[derive(Debug, Deserialize, PartialEq,Clone)]
struct LsBlkOutput {
    #[serde(rename = "blockdevices")]
    block_devices: Vec<LsBlkDeviceWithChildren>,
}
fn parse(input: &[u8]) -> Result<LsBlkOutput,()> {
    Ok(serde_json::from_slice(input).unwrap())
}
/// Struct for deserializing the JSON output of `lsblk`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq,Serialize)]
pub struct LsBlkDevice {
    pub name: Option<String>,
    pub fstype: Option<String>,
    pub fsver: Option<String>,
    pub label: Option<String>,
    pub uuid: Option<String>,
    pub fsavail: Option<u64>,
    pub fsused: Option<u64>,
    pub mountpoint:Option<String>,
    pub hotplug: bool,
    pub size: u64,
    pub vendor: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "rm")]
    pub is_removable: bool,
    pub state: Option<String>,
    #[serde(rename = "type")]
    pub device_type: String,
    pub kname: Option<String>
}
fn flattened(parsed:LsBlkOutput) -> Vec<LsBlkDevice> {
    let mut output = Vec::new();
    let mut stack = parsed.block_devices;
    while let Some(device) = stack.pop() {
        // output.push(device.details);
        // stack.extend(device.children);
        output.push(device.details.clone());
        for mut child in device.children.clone() {
            child.details.vendor=device.details.vendor.clone();
            child.details.model=device.details.model.clone();
            stack.push(child);

        }
    }
    // println!("output------{:?}",output);
    output
}

 pub fn get_lsblk_devices() -> Result<Vec<LsBlkDevice>,()> {
    let output = get_lsblk_output()?;
    let parsed =parse(&output)?;
    // println!("{:?}",parsed.clone());
    Ok(flattened(parsed))
}
 /// Get information about all disk devices.
 /// 
 struct diskinfo{
    name:String,
 }
pub fn get_disks() -> Result<(Vec<LsBlkDevice>,Vec<LsBlkDevice>),()> {
// fn get_disks() -> Result<Vec<PathBuf>,()> {
    let devices = get_lsblk_devices().expect("Unable to get block devices");
    let mut disks = Vec::new();
    let mut uddisks = Vec::new();
    for device in devices {
        if(device.uuid.is_some() && device.label.is_some()){
            disks.push(device.clone());
        }
        else if device.fstype.is_some(){
            disks.push(device.clone());
        }
        else{

            uddisks.push(device.clone());
        }
        // let ps=device.mountpoints.get(0).unwrap().clone().unwrap();
        // disks.push(ps);
        // disks.push(Path::new(&ps).into());
    }
    Ok((disks,uddisks))
}

 #[test]
 fn drllt(){
    // let output = get_lsblk_output().unwrap();
    // println!("{}",String::from_utf8(output.clone()).unwrap());
    // let parsed = parse(&output).unwrap();
    // println!("{:?}",flattened(parsed));

    println!("{:?}",get_disks().unwrap().1);

    // mountdrive("96A6-580C".to_string(), "/dev/nvme0n1p5".to_string());



    // let dv=get_disks().unwrap().0;
    // // println!("{:?}",serde_json::to_value(dv.clone()));
    // // println!("{:?}",dv);
    // for ed in dv{
    //     println!("{:?}",ed);
    // }
    // println!("-----------------------");
    // println!("-----------------------");
    // println!("-----------------------");
    // println!("-----------------------");
    // let dv=get_disks().unwrap().1;
    // for ed in dv{
    //     println!("{:?}",ed);
    // }



    // println!("{:?}",get_disks())
    // let output  =get_lsblk_output();
    // println!("{}",output.unwrap())
    // for ed in dl{
    //     println!("{:?}",ed);

    //     // println!("{:?}",ed.description);
    // }
 }





 pub fn get_drives() -> Result<Drives, String> {
    let mut sys= System::new();
    sys.refresh_disks_list();
    let array_of_drives =sys
        .disks()
        .iter()
        .map(|disk| {
            let mut total = sizeunit::size(disk.total_space(),true);
            let free = sizeunit::size(disk.available_space(),true);
            let mount_point = disk.mount_point().to_str().unwrap_or("/").to_string();
            let name = disk.name().to_str().unwrap_or("Disk").to_string();
            let is_removable = disk.is_removable();
           
            let file_system = String::from_utf8(disk.file_system().to_vec())
                .unwrap_or_else(|_| "Unknown File System".to_string());
            let disk_type = 
            match disk.kind() {
                sysinfo::DiskKind::SSD => "SSD".to_string(),
                sysinfo::DiskKind::HDD => "HDD".to_string(),
                _ => "Removable Disk".to_string(),
            };

            DriveInformation {
                name:if !cfg!(unix) {
                    name.clone()
                }
                else{
                    mount_point.clone()
                },
                mount_point:if !cfg!(unix) {
                    mount_point
                }
                else{
                    name
                },
                total,
                free,
                is_removable,
                disk_type,
                file_system,
            }
        })
        .collect();

    Ok(Drives { array_of_drives })
}

 pub fn mountdrive(uuid:String,mount_point:String) ->bool{
    let mount_cmd=Command::new("udisksctl")
                .arg("mount")
                .arg("--block-device")
                .arg(&mount_point)
                .status()
                .expect(&format!("failed to run udisksctl"));
    (mount_cmd.success())
 } 
 pub fn unmountdrive(uuid:String,mount_point:String) ->bool{
    let mount_cmd=Command::new("udisksctl")
                .arg("unmount")
                .arg("--block-device")
                .arg(&mount_point)
                .status()
                .expect(&format!("failed to run udisksctl"));
    (mount_cmd.success())
 }
#[test]
fn test_result(){
//   println!("{:?}",get_drives().unwrap().array_of_drives);

let dn:Vec<String>=get_drives().unwrap().array_of_drives.iter().map(|ed|{
    ed.mount_point.clone()
  }).collect();
  println!("{:?}",dn);
}