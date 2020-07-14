# twm
> A tiling window manager for Windows, written in Rust.

> __Note:__ Currently only the `twm-core` crate is uploaded to the repository as I am still debating and tinkering around with the final API. Once I like it, the full project will be uploaded to the repository.

## Modules

| Name | Description |
|---|---|
| twm-cli | A CLI program that can be used to query information from the running twm instance. The data is returned as `json`. |
| twm-core | The core logic and API of twm. The code is platform independent. |
| tmw-main | The running twm instance, implemented as a Windows service. |
| twm-protocol | The RPC protocol that the `twm-cli` and `twm-main` use to communicate. |
