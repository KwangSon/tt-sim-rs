# tt-sim-rs

Virtual Tenstorrent Device Simulator

## Architecture

```
tt-sim-rs (userspace simulator)
  ↓ (Shared Memory via mmap)
  └─ /tmp/tt-sim-rs-<device_id>
  
tt-kmd (kernel driver - simulation mode)
  ↓ (ioctl)
luwen (userspace library)
  ↓
User applications (tt-metal, tools)
```
