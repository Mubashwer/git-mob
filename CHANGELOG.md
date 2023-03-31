# Changelog

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
