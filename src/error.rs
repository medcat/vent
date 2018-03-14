error_chain! {
    errors {
        KvmSystemOpenError
        KvmSystemOperationError
        KvmCapabilityCheckError
        KvmMissingCapabilityError
        KvmMachineOperationError
        KvmCoreOperationError
    }
}
