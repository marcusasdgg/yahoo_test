## [0.1.1] - 2024-5-28
### Changed
- Changed type of internal field cookie and crumb to tokio::sync formerly arc::sync to support running the object in multithreaded async instances.


### Removed
- removed unused libraries in source file.

## [0.1.2] - 2024-5-28
### Changed
- fixed bug caused by starts with that broke the entire API.


## [0.1.3] - 2024-5-28
### Changed
- fixed bug in the error searching to finally fix returning an error if ticker was not found.

## [0.1.4] - 2024-5-29
### Changed
- added option to declare exchange i.e for ASX, that internally appends neccesary string to search Yahoo's API.

## [0.1.4] - 2024-5-29
### Changed
- removed option to declare exchange, that will be handled by user.
- hard coded type for YAHOO connect return to be of type queryresponse struct for easier field handling.

