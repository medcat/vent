error_chain!{
    foreign_links {
        KvmError(::kvm::Error);
    }

    errors {
        InvalidFirmwareError(reason: &'static str) {
            description("could not load instance firmware")
            display("could not load instance firmware: {}", reason)
        }
    }
}
