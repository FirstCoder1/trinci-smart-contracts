ChangeLog
=========

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com).

Type of changes

* Added: for new features.
* Changed: for changes in existing functionality.
* Deprecated: for soon-to-be removed features.
* Removed: for now removed features.
* Fixed: for any bug fixes.
* Security: in case of vulnerabilities.

This project adheres to [Semantic Versioning](http://semver.org).

Given a version number MAJOR.MINOR.PATCH
* MAJOR incremented for incompatible API changes
* MINOR incremented for new functionalities
* PATCH incremented for bug fixes

Additional labels for pre-release metadata:
* alpha.x: internal development stage.
* beta.x: shipped version under testing.
* rc.x: stable release candidate.

0.2.1 - 13-10-2021
------------------

Changed 
* replaced HashMap with BTreeMap
* fixed penalty_asset handling


0.2.0 - 18-09-2021
------------------

Changed
* fees calculation: now are expressed in thousandth (e.g. fee: 50 => 5%)
* exchange rates: now are expressed in hundredth (e.g. rate: 250 => asset2 = 2.5 * asset1)


0.1.0 - 01-09-2021
------------------

Added
* `init` method - initializes the contract
* `get_info` method - retrieves the contract information
* `apply` method - allows to apply to the contract to exchange an asset with another
* `abort` method - allows to prematurely end the contract
