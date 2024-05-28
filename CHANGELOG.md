## [0.1.1] - 2024-5-28
### Changed
- Changed type of internal field cookie and crumb to tokio::sync formerly arc::sync to support running the object in multithreaded async instances.


### Removed
- removed unused libraries in source file.