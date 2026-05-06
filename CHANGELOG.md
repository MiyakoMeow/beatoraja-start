# Changelog
## [0.2.2](https://github.com/MiyakoMeow/beatoraja-start/compare/v0.2.1...v0.2.2) - 2026-04-25


### Bug Fixes

- **(ci)** run release-plz on windows to fix cargo package verification ([#20](https://github.com/MiyakoMeow/beatoraja-start/pull/20))
- **(ci)** disable semver_check to avoid cargo package failure on linux CI ([#19](https://github.com/MiyakoMeow/beatoraja-start/pull/19))
- **(ci)** add git_only=true to use git tags for version detection ([#18](https://github.com/MiyakoMeow/beatoraja-start/pull/18))
- **(ci)** remove persist-credentials false from release-plz workflow ([#17](https://github.com/MiyakoMeow/beatoraja-start/pull/17))
- **(ci)** add release_commits to trigger release PRs on refactor/chore commits ([#14](https://github.com/MiyakoMeow/beatoraja-start/pull/14))


### Chore

- **(ci)** revert unnecessary release-plz config changes ([#22](https://github.com/MiyakoMeow/beatoraja-start/pull/22))
- add stdin null to can_run for cleaner process spawning ([#15](https://github.com/MiyakoMeow/beatoraja-start/pull/15))
- **(ci)** enable release_always to trigger release PRs without feat/fix commits ([#13](https://github.com/MiyakoMeow/beatoraja-start/pull/13))
- **(deps)** bump actions/checkout from 4 to 6 ([#9](https://github.com/MiyakoMeow/beatoraja-start/pull/9))
- **(deps)** bump EmbarkStudios/cargo-deny-action from 2.0.13 to 2.0.17 ([#8](https://github.com/MiyakoMeow/beatoraja-start/pull/8))


### Features

- simplify resolve_executable with unwrap_or ([#16](https://github.com/MiyakoMeow/beatoraja-start/pull/16))
- add cross-platform support for non-Windows systems ([#10](https://github.com/MiyakoMeow/beatoraja-start/pull/10))


### Refactoring

- **(ci)** split CI checks into separate jobs and extract cargo-deny ([#12](https://github.com/MiyakoMeow/beatoraja-start/pull/12))
- remove redundant and risky JVM args for Java 21+ ([#11](https://github.com/MiyakoMeow/beatoraja-start/pull/11))
- migrate CI from release-please to release-plz and renovate to dependabot ([#7](https://github.com/MiyakoMeow/beatoraja-start/pull/7))
## [0.2.4](https://github.com/MiyakoMeow/beatoraja-start/compare/v0.2.3...v0.2.4) - 2026-04-25


### Documentation

- sync JVM args in README with actual implementation ([#26](https://github.com/MiyakoMeow/beatoraja-start/pull/26))


## [0.2.3](https://github.com/MiyakoMeow/beatoraja-start/compare/v0.2.2...v0.2.3) - 2026-04-25


### Chore

- **(ci)** integrate build and upload into release-plz workflow ([#24](https://github.com/MiyakoMeow/beatoraja-start/pull/24))


## [0.2.1](https://github.com/MiyakoMeow/beatoraja-start/compare/v0.2.0...v0.2.1) (2025-11-16)


### Features

* **ci:** split release scripts ([a249195](https://github.com/MiyakoMeow/beatoraja-start/commit/a249195efa3bb3b45f75068b8e72c9ce7418ebcb))
* **cmd:** check java executable path ([bc3fa0b](https://github.com/MiyakoMeow/beatoraja-start/commit/bc3fa0b1043c9af3480d8b6b83b250fa8e5c4642))


### Bug Fixes

* **ci:** use helper-bot for release-please ([4ca6faa](https://github.com/MiyakoMeow/beatoraja-start/commit/4ca6faa6e61c7161aab44cdc58b16305bdf0d091))
* **cmd:** remove UseLargePages ([8aeea40](https://github.com/MiyakoMeow/beatoraja-start/commit/8aeea40c057a74935d7d48498ba873de76906249))


### Miscellaneous Chores

* change release ([59c6601](https://github.com/MiyakoMeow/beatoraja-start/commit/59c6601b7ed88007f37349778af52b5136dc0dc8))

## [0.2.0](https://github.com/MiyakoMeow/beatoraja-start/compare/v0.1.2...v0.2.0) (2025-11-11)


### Features

* **ci:** publish builds after release ([56badd8](https://github.com/MiyakoMeow/beatoraja-start/commit/56badd80db513f1618eaaf1566ff8063de180c73))
* enhance args ([3528add](https://github.com/MiyakoMeow/beatoraja-start/commit/3528add5cc449799c30c4164fcebdaab26cc42d2))


### Bug Fixes

* clippy ([d5aca45](https://github.com/MiyakoMeow/beatoraja-start/commit/d5aca453d357567ff3add83d6b53cde954a3ebda))
