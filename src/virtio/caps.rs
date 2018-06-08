#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Capability {
    cap_vndr: u8,
    cap_next: u8,
    cap_len: u8,
    cfg_type: u8,
    bar: u8,
    _pad: [u8; 3],
    offset: u32,
    length: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Common {
    /// The driver uses this to select which feature bits
    /// [`device_feature`] shows. Value `0x0` selects Feature Bits 0 to
    /// 31, `0x1` selects Feature Bits 32 to 63, etc.
    ///
    /// The driver may read and write to this.
    device_feature_select: u32,
    /// The device uses this to report which feature bits it is
    /// offering to the driver: the driver writes to
    /// [`device_feature_select`] to select which feature bits are
    /// presented.
    ///
    /// The driver may only read from this.
    device_feature: u32,
    /// The driver uses this to select which feature bits
    /// `driver_feature` shows. Value `0x0` selects Feature Bits 0 to
    /// 31, `0x1` selects Feature Bits 32 to 63, etc.
    ///
    /// The driver may read and write to this.
    driver_feature_select: u32,
    /// The driver writes this to accept feature bits offered by the
    /// device. Driver Feature Bits selected by driver_feature_select.
    ///
    /// The driver may read and write to this.
    driver_feature: u32,
    /// The driver sets the Configuration Vector for MSI-X.
    ///
    /// The driver may read and write to this.
    msix_config: u16,
    /// The device specifies the maximum number of virtqueues
    /// supported here.
    ///
    /// The driver may only read from this.
    num_queues: u16,
    /// The driver writes the device status here. Writing 0 into this
    /// field resets the device.
    ///
    /// The driver may read and write to this.
    device_status: u8;
    /// Configuration atomicity value. The device changes this every
    /// time the configuration noticeably changes.
    ///
    /// The driver may only read from this.
    config_generation: u8;

    /* About a specific virtqueue. */
    /// Queue Select. The driver selects which virtqueue the following
    /// fields refer to.
    ///
    /// The driver may read and write to this.
    queue_select: u16,
    /// Queue Size. On reset, specifies the maximum queue size
    /// supported by the hypervisor. This can be modified by driver to
    /// reduce memory requirements. A 0 means the queue is unavailable.
    ///
    /// The driver may read and write to this.  It must be either a
    /// power of two, or zero.
    queue_size: u16,
    /// The driver uses this to specify the queue vector for MSI-X.
    ///
    /// The driver may read and write to this.
    queue_msix_vector: u16,
    /// The driver uses this to selectively prevent the device from
    /// executing requests from this virtqueue. 1 - enabled;
    /// 0 - disabled.
    ///
    /// The driver may read and write to this.
    queue_enable: u16,
    /// The driver reads this to calculate the offset from start of
    /// Notification structure at which this virtqueue is located.
    /// Note: this is not an offset in bytes.
    ///
    /// The driver may only read from this.
    queue_notify_off: u16,
    /// The driver writes the physical address of Descriptor Table
    /// here.
    ///
    /// The driver may read and write to this.
    queue_desc: u64,
    /// The driver writes the physical address of Available Ring here.
    ///
    /// The driver may read and write to this.
    queue_avail: u64,
    /// The driver writes the physical address of Used Ring here.
    ///
    /// The driver may read and write to this.
    queue_used: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Notify {
    cap: Capability,
    notify_off_multiplier: u32,
}
