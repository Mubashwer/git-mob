# Changelog

## [1.9.2](https://github.com/Mubashwer/git-mob/compare/v1.9.1...v1.9.2) (2025-07-01)


### Bug Fixes

* resolve path injection vulnerabilities in setup command ([3acf7a9](https://github.com/Mubashwer/git-mob/commit/3acf7a9f50a8071dd35579cac1e2983245a209d8))

## [1.9.1](https://github.com/Mubashwer/git-mob/compare/v1.9.0...v1.9.1) (2025-06-17)


### Bug Fixes

* update dependencies to resolve yanked futures-util security warning ([ebb739b](https://github.com/Mubashwer/git-mob/commit/ebb739b10919b5e89edec7aac59d2cfcc26905f5))

## [1.9.0](https://github.com/Mubashwer/git-mob/compare/v1.8.0...v1.9.0) (2025-06-14)


### Features

* **mob:** add functionality to include non-team members in the mob session ([845ec2c](https://github.com/Mubashwer/git-mob/commit/845ec2c4694835c5b94a2a08e1978667d0d2016d))

## [1.8.1](https://github.com/Mubashwer/git-mob/compare/v1.8.0...v1.8.1) (2025-06-14)


### Bug Fixes

* improve error messages in local prepare-commit-msg hook ([bb5eefa](https://github.com/Mubashwer/git-mob/commit/bb5eefaaad2b3718c3226857dd336c420002314c))
* update terminology from "CLI app" to "CLI tool" ([6c968cb](https://github.com/Mubashwer/git-mob/commit/6c968cb56ef64a5a9697aad359d6c2d98a92441c))

## [1.8.0](https://github.com/Mubashwer/git-mob/compare/v1.7.0...v1.8.0) (2025-06-10)


### Features

* make --global the default behavior for setup command ([#100](https://github.com/Mubashwer/git-mob/issues/100)) ([72a41b9](https://github.com/Mubashwer/git-mob/commit/72a41b9e96217a96effc555fd16a48a710337f27))

## [1.7.0](https://github.com/Mubashwer/git-mob/compare/v1.6.2...v1.7.0) (2025-06-06)


### Features

* rename coauthor command to team-member for improved clarity ([#98](https://github.com/Mubashwer/git-mob/issues/98)) ([9a3c4a8](https://github.com/Mubashwer/git-mob/commit/9a3c4a826b26e6ce8b47496d53244528bfe35551))

## [1.6.2](https://github.com/Mubashwer/git-mob/compare/v1.6.1...v1.6.2) (2024-04-19)


### Bug Fixes

* add support for release bin installation with cargo-binstall ([54f523b](https://github.com/Mubashwer/git-mob/commit/54f523bbc2fc258452ae249acfa6ce524cde545f))
* **prepare-commit-msg.local:** fix error msg when git hooks dir doesn't exist ([28dac39](https://github.com/Mubashwer/git-mob/commit/28dac3950c3d6bb8edefa9c0505cf0580ba7a828))

## [1.6.1](https://github.com/Mubashwer/git-mob/compare/v1.6.0...v1.6.1) (2024-04-16)


### Bug Fixes

* **setup:** create githook in correct path when hooks dir starts with ~ ([a741e2f](https://github.com/Mubashwer/git-mob/commit/a741e2fb0c6f7609ac7cf2b17d664f8cd598bf24))

## [1.6.0](https://github.com/Mubashwer/git-mob/compare/v1.5.4...v1.6.0) (2024-04-07)


### Features

* add setup subcommand to automate `prepare-commit-msg` githook setup ([#58](https://github.com/Mubashwer/git-mob/issues/58)) ([8077a21](https://github.com/Mubashwer/git-mob/commit/8077a213c6fdd84937184a387474cb3548538237))

## [1.5.4](https://github.com/Mubashwer/git-mob/compare/v1.5.3...v1.5.4) (2024-03-29)


### Bug Fixes

* improve error msg when adding coauthor with invalid key ([81fd16d](https://github.com/Mubashwer/git-mob/commit/81fd16d03a91cdda98a42c14d3b7c9f2b386f061))

## [1.5.3](https://github.com/Mubashwer/git-mob/compare/v1.5.2...v1.5.3) (2024-03-28)


### Bug Fixes

* **coauthor_repo:** improve error handling ([72b5e32](https://github.com/Mubashwer/git-mob/commit/72b5e3289cb1c3ef766d1575cc107150e084e14c))

## [1.5.2](https://github.com/Mubashwer/git-mob/compare/v1.5.1...v1.5.2) (2024-03-27)


### Bug Fixes

* return non-zero exit code in expected error cases ([215586a](https://github.com/Mubashwer/git-mob/commit/215586ac5cf0d84d3de72a1369d268dc2084f62d))

## [1.5.1](https://github.com/Mubashwer/git-mob/compare/v1.5.0...v1.5.1) (2024-03-22)


### Bug Fixes

* stop error msg when escaping git mob --with ([ed04a38](https://github.com/Mubashwer/git-mob/commit/ed04a389bb269938b7d7e22b8dcfbe3f469dffc0))

## [1.5.0](https://github.com/Mubashwer/git-mob/compare/v1.4.0...v1.5.0) (2024-03-21)


### Features

* add git mob --trailers command ([17e35cf](https://github.com/Mubashwer/git-mob/commit/17e35cf862bcc11b8a333629ff0fba3b0edbd0de))

## [1.4.0](https://github.com/Mubashwer/git-mob/compare/v1.3.1...v1.4.0) (2023-04-21)


### Features

* add support for aarch64-unknown-linux-musl ([aa458cb](https://github.com/Mubashwer/git-mob/commit/aa458cbc758527f120e10a69d62133ecb1e1ec84))

## [1.3.1](https://github.com/Mubashwer/git-mob/compare/v1.3.0...v1.3.1) (2023-04-15)


### Bug Fixes

* remove building binary for aarch64 linux ([d4c3604](https://github.com/Mubashwer/git-mob/commit/d4c3604d3812778c8bd964d01826c43cf8ebef58))
* remove building binary for aarch64 linux-musl ([65ee650](https://github.com/Mubashwer/git-mob/commit/65ee650ab809cb3c70c3eb72cedced4e9b068364))

## [1.3.0](https://github.com/Mubashwer/git-mob/compare/v1.2.3...v1.3.0) (2023-04-15)


### Features

* add binaries for more platforms ([decf42b](https://github.com/Mubashwer/git-mob/commit/decf42bfd807734e772e3c8900d64f6d27bb913c))

## [1.2.3](https://github.com/Mubashwer/git-mob/compare/v1.2.2...v1.2.3) (2023-04-15)


### Bug Fixes

* set correct path when archiving release binary ([36924c1](https://github.com/Mubashwer/git-mob/commit/36924c117f7eee652fad72df708f16cda916e624))

## [1.2.2](https://github.com/Mubashwer/git-mob/compare/v1.2.1...v1.2.2) (2023-04-15)


### Bug Fixes

* set correct path when archiving release binary ([a5e4956](https://github.com/Mubashwer/git-mob/commit/a5e49560a8a94273ce64b22414aa9146a20d8cf3))

## [1.2.1](https://github.com/Mubashwer/git-mob/compare/v1.2.0...v1.2.1) (2023-04-15)


### Bug Fixes

* set target correctly when building release binary ([38497e9](https://github.com/Mubashwer/git-mob/commit/38497e942afdd27a9239cc88121dea84e0f3d46a))

## [1.2.0](https://github.com/Mubashwer/git-mob/compare/v1.1.7...v1.2.0) (2023-04-15)


### Features

* add support for more platforms ([031a9f8](https://github.com/Mubashwer/git-mob/commit/031a9f8aacaaf0d579f13e62fbf6554d52fae674))

## [1.1.7](https://github.com/Mubashwer/git-mob/compare/v1.1.6...v1.1.7) (2023-04-15)


### Bug Fixes

* stop clearing mob when quitting -with prompt ([d45cafc](https://github.com/Mubashwer/git-mob/commit/d45cafc5523c0ccd15e39d1166de6b81bd06c52c))

## [1.1.6](https://github.com/Mubashwer/git-mob/compare/v1.1.5...v1.1.6) (2023-04-02)


### Bug Fixes

* **prepare-commit-msg:** append co-author-trailers correctly when adding jira prefix ([dd4775a](https://github.com/Mubashwer/git-mob/commit/dd4775a3eda55e58fe542ee0bd2057edbea6fb4e))

## [1.1.5](https://github.com/Mubashwer/git-mob/compare/v1.1.4...v1.1.5) (2023-04-02)


### Bug Fixes

* make commands module private ([2c544a7](https://github.com/Mubashwer/git-mob/commit/2c544a7c95dee44f61cd6c03aa3f8f408fa29eef))

## [1.1.4](https://github.com/Mubashwer/git-mob/compare/v1.1.3...v1.1.4) (2023-04-01)


### Bug Fixes

* **ci:** fix uploading archived binary to github release ([629fde8](https://github.com/Mubashwer/git-mob/commit/629fde8cd18f9beb8498753be93a33434ae978fe))

## [1.1.3](https://github.com/Mubashwer/git-mob/compare/v1.1.2...v1.1.3) (2023-04-01)


### Bug Fixes

* **ci:** set correct directory when archiving release binary ([632a9f1](https://github.com/Mubashwer/git-mob/commit/632a9f1fc38bedeba118241c3e270693b15d967c))

## [1.1.2](https://github.com/Mubashwer/git-mob/compare/v1.1.1...v1.1.2) (2023-04-01)


### Bug Fixes

* zip binaries in github release ([b108084](https://github.com/Mubashwer/git-mob/commit/b108084dd878ebeb145eab02fe9f9fc1c1d37aef))

## [1.1.1](https://github.com/Mubashwer/git-mob/compare/v1.1.0...v1.1.1) (2023-04-01)


### Bug Fixes

* remove set -u from prepare-commit-msg ([49b4cc8](https://github.com/Mubashwer/git-mob/commit/49b4cc8564ffb674844c143a409a676c308b02d6))

## [1.1.0](https://github.com/Mubashwer/git-mob/compare/v1.0.1...v1.1.0) (2023-03-31)


### Features

* prevent adding of duplicate co-authored-by trailers ([3125a11](https://github.com/Mubashwer/git-mob/commit/3125a117fbd0af241b29ad09fe98bad1d4d9b63a))


### Bug Fixes

* change crate name to git-mob-tool as git-mob already exists ([203c7cf](https://github.com/Mubashwer/git-mob/commit/203c7cfe1847b15464a77dceca3b1c63343af4c6))

## [1.0.1](https://github.com/Mubashwer/git-mob/compare/v1.0.0...v1.0.1) (2023-03-31)


### Bug Fixes

* remove some crate keywords as crates.io only allows 5 ([386bf4e](https://github.com/Mubashwer/git-mob/commit/386bf4eba9788e6d608e1eb5027c3f02e8068dd7))

## [1.0.0](https://github.com/Mubashwer/git-mob/compare/v0.1.2...v1.0.0) (2023-03-31)


### ⚠ BREAKING CHANGES

* first release

### Features

* update prepare-commit-msg ([31e181b](https://github.com/Mubashwer/git-mob/commit/31e181bf62d7d925ea592dafe5647d856b6d9a95))


### Bug Fixes

* **prepare-commit-msg:** add empty lines before coauthor trailers ([31509c4](https://github.com/Mubashwer/git-mob/commit/31509c40e50d3a5149cbea95b8c44762e6a4ba3c))
* stop printing empty lines when coauthor lists are empty ([ba2f0c6](https://github.com/Mubashwer/git-mob/commit/ba2f0c6e7936e8fd5b248cb4727b83e81c441dc7))


### Continuous Integration

* publish to cargo on release ([f0a554a](https://github.com/Mubashwer/git-mob/commit/f0a554ab9a4dff54eb024afd04ee16087520a212))

## [0.1.2](https://github.com/Mubashwer/git-mob/compare/v0.1.1...v0.1.2) (2023-03-23)


### Bug Fixes

* try to fix triggering of cargo publish ([0181e27](https://github.com/Mubashwer/git-mob/commit/0181e27842bbecc7b057e08caf0a23ddc22da736))

## [0.1.1](https://github.com/Mubashwer/git-mob/compare/v0.1.0...v0.1.1) (2023-03-23)


### Bug Fixes

* test commit to test release ([9a319cb](https://github.com/Mubashwer/git-mob/commit/9a319cb96ef4483f020efb00a2aa6b101f400c12))

## 0.1.0 (2023-03-23)


### Features

* add 'with' sub-command ([3f98cb5](https://github.com/Mubashwer/git-mob/commit/3f98cb5bfec5d63eb419d5b895a59004c7d1db3e))
* add clear opt with test ([ab96375](https://github.com/Mubashwer/git-mob/commit/ab963752d15124880bfa361175c355da8def3e7f))
* add coauthor command with list opt ([7875e5d](https://github.com/Mubashwer/git-mob/commit/7875e5d80e19cc1c7f34e4a57ad67f507fd7a1bd))
* add list mob coauthors command ([0651246](https://github.com/Mubashwer/git-mob/commit/065124682b6a45f59059e607f83db349e8068ba1))
* add with subcommand ([e860988](https://github.com/Mubashwer/git-mob/commit/e8609882d42ceed6e27aa3ce0e93a4e68bcad8ac))
* implement handling --add option for coauthor subcommand ([0071ef3](https://github.com/Mubashwer/git-mob/commit/0071ef31310132cced2a4574ecfdd946d28bfd28))
* implement handling of delete opt of coauthor command ([4a15d54](https://github.com/Mubashwer/git-mob/commit/4a15d54b7ddce2c77336f3e02ee3eecc271e4ab9))
* show keys when listing all co-authors ([8e98178](https://github.com/Mubashwer/git-mob/commit/8e9817862fba21859756a52863a3cd7280b256ef))


### Bug Fixes

* improve error handling for --with ([b044eec](https://github.com/Mubashwer/git-mob/commit/b044eec50ebc67ad62012f32f64c86e391996eef))
* set  license in Cargo.toml ([c1bb147](https://github.com/Mubashwer/git-mob/commit/c1bb1476fd932022ece5b2f4bccf86ea4399377e))
* show help when no option/command is passed ([1c0919f](https://github.com/Mubashwer/git-mob/commit/1c0919fdbc0a50e0370005c20dbaff2c2cce2be1))
