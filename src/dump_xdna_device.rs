// for debug

use libamdgpu_top::{stat, xdna};

pub fn dump_xdna_device(title: &str) {
    println!("{title}\n");

    let Some(xdna_device) = xdna::find_xdna_device() else {
        println!("There are no the XDNA NPU devices found.");
        return;
    };

    println!("{xdna_device:#X?}");

    if let Ok(fw_ver) = xdna_device.get_xdna_fw_version() {
        println!("FW Version: {fw_ver}");
    }

    // for fdinfo test
    let fd = xdna_device.get_fd().unwrap();
    let mut xdna_proc_index = xdna_device.arc_proc_index.lock().unwrap();

    stat::update_index_by_all_proc(
        &mut xdna_proc_index,
        &[&xdna_device.accel],
        &stat::get_process_list(),
    );

    let mut xdna_fdinfo = xdna::XdnaFdInfoStat::default();
    xdna_fdinfo.get_all_proc_usage(&xdna_proc_index);

    println!("{:#?}", xdna_fdinfo.proc_usage);

    if let Ok(s) = std::fs::read_to_string(format!("/proc/self/fdinfo/{fd}")) {
        if !s.contains("drm-engine-npu-amdxdna:") {
            println!("This amdxdna driver version dose not support DRM client usage stats.");
        }
        println!("fdinfo (raw):\n{s}");
    }

    println!("{:#?}", xdna::get_xdna_clock_metadata(fd));
    println!("{:#?}", xdna::get_xdna_hardware_version(fd));
    println!("{:#?}", xdna::get_xdna_firmware_version(fd));
    println!("{:#?}", xdna::get_xdna_power_mode(fd));
    println!("{:#?}", xdna::get_xdna_metadata(fd));
}
