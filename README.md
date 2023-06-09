# AMDGPU\_TOP
`amdgpu_top` is tool that display AMD GPU utilization, like [umr](https://gitlab.freedesktop.org/tomstdenis/umr/) or [clbr/radeontop](https://github.com/clbr/radeontop) or [intel_gpu_top](https://gitlab.freedesktop.org/drm/igt-gpu-tools/-/blob/master/man/intel_gpu_top.rst).  
The tool displays information gathered from performance counters (GRBM, GRBM2), sensors, fdinfo, and AMDGPU driver.  

| Simple TUI<br>(like nvidia-smi, rocm-smi) | TUI | GUI |
| :-: | :-: | :-: |
| ![amdgpu_top Simple TUI](https://github.com/Umio-Yasuno/amdgpu_top/assets/53935716/46c62caf-8137-4f72-a1f4-2ca395a9cb5c) | ![amdgpu_top TUI](https://github.com/Umio-Yasuno/amdgpu_top/assets/53935716/17318061-d8db-4157-a13e-67006388f587) | ![amdgpu_top GUI mode](https://user-images.githubusercontent.com/53935716/235711189-3379b4c2-8371-4760-a9e3-5f2c3e72e9c4.png)

## Usage
```
cargo run -- [options ..]
```

### Option
```
FLAGS:
   -d, --dump
       Dump AMDGPU info (Specifications, VRAM, PCI, ResizableBAR, VBIOS, Video caps)
   --list
       Display a list of AMDGPU devices (can be combined with "-d" option)
   -J, --json
       Output JSON formatted data
   --gui
       Launch GUI mode
   --smi
       Launch Simple TUI mode (like nvidia-smi, rocm-smi)
   -h, --help
       Print help information

OPTIONS:
   -i <u32>
       Select GPU instance
   --pci <String>
       Specifying PCI path (domain:bus:dev.func)
   -u <u64>, --update-process-index <u64>
       Update interval in seconds of the process index for fdinfo (default: 5s)
```

### Commands for TUI
| key |                                     |
| :-- | :---------------------------------: |
| f   | toggle fdinfo                       |
| n   | toggle Sensors                      |
| m   | toggle GPU Metrics                  |
| h   | change update interval (high = 100ms, low = 1000ms) |
| q   | Quit                                |
| P   | sort fdinfo by pid                  |
| M   | sort fdinfo by VRAM usage           |
| G   | sort fdinfo by GFX usage            |
| M   | sort fdinfo by MediaEngine usage    |
| R   | reverse sort                        |

<!-- | g   | toggle GRBM                         | -->
<!-- | r   | toggle GRBM2                        | -->
<!-- | v   | toggle VRAM/GTT Usage               | -->

## Installation
### Packages
 * [Releases](https://github.com/Umio-Yasuno/amdgpu_top/releases/latest)
   * .deb (generated by [cargo-deb](https://github.com/kornelski/cargo-deb))
   * .rpm (generated by [cargo-generate-rpm](https://github.com/cat-in-136/cargo-generate-rpm))
   * .AppImage (generated by [cargo-appimage](https://github.com/StratusFearMe21/cargo-appimage))
 * [AUR](https://aur.archlinux.org/packages/amdgpu_top)
 * [OpenMandriva](https://github.com/OpenMandrivaAssociation/amdgpu_top) to install run `sudo dnf install amdgpu_top`
 * [Nix](https://github.com/NixOS/nixpkgs/blob/master/pkgs/tools/system/amdgpu_top/default.nix)
### Build from source
Dependencies:
 * libdrm2
 * libdrm-amdgpu1

```
git clone https://github.com/Umio-Yasuno/amdgpu_top
cd amdgpu_top
cargo install --locked --path .
```

#### without GUI
```
cargo install --locked --path . --no-default-features --features="tui"
```

## Used library
 * [Cursive](https://github.com/gyscos/cursive)
 * [egui](https://github.com/emilk/egui)
 * [libdrm-amdgpu-sys-rs](https://github.com/Umio-Yasuno/libdrm-amdgpu-sys-rs)

## Reference
 * [Tom St Denis / umr · GitLab](https://gitlab.freedesktop.org/tomstdenis/umr/)
 * Mesa3D
    * [src/gallium/drivers/radeonsi/si_gpu_load.c · main · Mesa / mesa · GitLab](https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/gallium/drivers/radeonsi/si_gpu_load.c)
 * AMD Documentation
    * [R6xx_R7xx_3D.pdf](https://developer.amd.com/wordpress/media/2013/10/R6xx_R7xx_3D.pdf)
    * [CIK_3D_registers_v2.pdf](http://developer.amd.com/wordpress/media/2013/10/CIK_3D_registers_v2.pdf)
    * [MI200 Performance Counters: Listing](https://docs.amd.com/bundle/AMD-Instinct-MI200-Performance-Counters-v5.3/page/MI200_Performance_Counters_Listing.html)
    * [MI200 Performance Counters: Abbreviations](https://docs.amd.com/bundle/AMD-Instinct-MI200-Performance-Counters-v5.3/page/MI200_Performance_Counters_Abbreviations.html)
 * <https://github.com/AMDResearch/omniperf/tree/v1.0.4/src/perfmon_pub>
 * <https://github.com/freedesktop/mesa-r600_demo>
 * [radeonhd:r6xxErrata](https://www.x.org/wiki/radeonhd:r6xxErrata/)
 * Linux Kernel AMDGPU Driver
    * libdrm_amdgpu API
        * `/drivers/gpu/drm/amd/amdgpu/amdgpu_kms.c`
    * `amdgpu_allowed_register_entry`
        * `/drivers/gpu/drm/amd/amdgpu/{cik,nv,vi,si,soc15,soc21}.c`

## Alternatives
If `amdgpu_top` is not enough for you or you don't like it, try the following applications.

 * [clbr/radeontop](https://github.com/clbr/radeontop)
    * View your GPU utilization, both for the total activity percent and individual blocks.
 * [Syllo/nvtop](https://github.com/Syllo/nvtop)
    * GPUs process monitoring for AMD, Intel and NVIDIA
 * [Tom St Denis / umr · GitLab](https://gitlab.freedesktop.org/tomstdenis/umr/)
    * User Mode Register Debugger for AMDGPU Hardware
 * [GPUOpen-Tools/radeon_gpu_profiler](https://github.com/GPUOpen-Tools/radeon_gpu_profiler)
    * for developer
    * Radeon GPU Profiler (RGP) is a tool from AMD that allows for deep inspection of GPU workloads. 
