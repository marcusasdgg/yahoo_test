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

## [0.1.4] - 2024-5-29
### Fixed
- queryResponse was not visible in prior version please do not use.

## [0.1.6] - 2024-5-29
### Fixed
- internal issues cayused 0.1.5 to be yanked please use this version.

## [0.1.6] - 2024-5-29
### Fixed
- derived clone for struct

## [1.0.0] - 2024-7-13
### added
- new enum system to store finance data, based off of either Option or Equity.
- added robust type system for that as well as error checking.
### fixed
- fixed previous version failing to get any ticker caused by deserialization error.

## [1.0.1] - 2024-7-13
### fixed
- fixed visibility for Finretypes

## [1.0.2] - 2024-7-13
### fixed
- added Clone trait to FinResult.
