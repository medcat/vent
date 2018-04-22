use kvm;

error_chain! {
    errors {
        KvmSystemOpenError
        KvmSystemOperationError
        KvmCapabilityCheckError
        MemoryMapError
        NotifyReadError
        NotifyWriteError
        EventFdCreateError
        EventFdReadError
        ActionNotError

        DeviceInvalidPortError(port: u16) {
            description("a device was given an invalid port IO request")
            display("a device was given an invalid port IO request on port {}", port)
        }

        KvmMissingCapabilityError(cap: kvm::capabilities::Capability) {
            description("a required capability was missing from the system")
            display("the required capability {} was missing from the system", cap)
        }

        KvmMachineOperationError

        KvmMachineMissingPrimaryCoreError {
            description("the machine was missing the primary core")
            display("the machine was missing the primary core")
        }

        KvmCoreOperationError
        KvmCoreUninitializedError

        KvmIllegalExitReasonError(reason: u32) {
            description("the Core exited for an illegal reason")
            display("the Core exited for the illegal reason {}", reason)
        }
    }
}
