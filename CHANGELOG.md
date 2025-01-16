# Changelog

All notable changes to this project will be documented in this file.

## [0.9.1](https://github.com/JQiue/waline-rs/compare/v0.9.0..0.9.1) - 2025-01-16

### 🐛 Bug Fixes

- Update the request to use the rustls-tls feature for fixing build errors - ([2b99c46](https://github.com/JQiue/waline-rs/commit/2b99c4613a11f8f93ed108b55b41cbe59de7103f))

## [0.9.0](https://github.com/JQiue/waline-rs/compare/v0.8.0..v0.9.0) - 2025-01-16

### 🚀 Features

- Integrate Akismet for spam detection in comments - ([26d2ca1](https://github.com/JQiue/waline-rs/commit/26d2ca1e8a5a405052d413c3b7b738b5982153ee))
- Extend JWT token expiration to 30 days - ([77a75cf](https://github.com/JQiue/waline-rs/commit/77a75cf04ed23cc96cfa84aa28dc6be655f677b3))
- Enhance logging configuration with custom filters - ([0fef15b](https://github.com/JQiue/waline-rs/commit/0fef15bedf579f7ef21e2a119ffa53d28249789b))
- Update configuration structure and improve logging - ([b81e34a](https://github.com/JQiue/waline-rs/commit/b81e34aeac9801c05ec95679f7e6c1790e15416b))
- Add login field to AppState and handle unauthorized access - ([98ee186](https://github.com/JQiue/waline-rs/commit/98ee1864cb647b7c54c64e156c10c94863213fb3))

### 🐛 Bug Fixes

- Improve comment handler reliability and security - ([adcf84e](https://github.com/JQiue/waline-rs/commit/adcf84eaa50bd4f73ed754f3ae47cddfd2680da0))
- Correct user type setting logic in service - ([906e5c8](https://github.com/JQiue/waline-rs/commit/906e5c89e4a1b06f551ea977d1962a4c0a78c88d))

### 🚜 Refactor

- Simplify JWT verification logic in comment handler - ([7414ad7](https://github.com/JQiue/waline-rs/commit/7414ad7b6d3d42e3d9427493fbab7d50efeddd89))
- Improve error handling in user retrieval logic - ([b1bdb0a](https://github.com/JQiue/waline-rs/commit/b1bdb0aced889f0d756e34b51aa22aaf290b168f))
- Standardize error handling and enhance comment deletion logic - ([74963f3](https://github.com/JQiue/waline-rs/commit/74963f3ea6fda128806b69852e8130253b47f32e))

### ⚙️ Miscellaneous Tasks

- *(docker)* Add Dockerfile and .dockerignore for deployment - ([c775278](https://github.com/JQiue/waline-rs/commit/c7752784cd9a2ff5c46382828ca750564560787c))
- Update README and add Chinese translation - ([d92669b](https://github.com/JQiue/waline-rs/commit/d92669b0e438602dfae917c0bad91d743c2b7c07))
- Update copyright year in LICENSE file - ([8842b85](https://github.com/JQiue/waline-rs/commit/8842b85811c62d7f06dab093f1b04e64944824cf))
- Update README to include Shuttle deployment instructions - ([c67926a](https://github.com/JQiue/waline-rs/commit/c67926aadb3f4dbced034889196899a6de5ea7ea))
- Release waline-mini version 0.9.0 - ([1001811](https://github.com/JQiue/waline-rs/commit/10018115a3466b0d114bfb7db648f79c3467c2fe))

## [0.8.0](https://github.com/JQiue/waline-rs/compare/v0.7.1..v0.8.0) - 2025-01-07

### 🚀 Features

- Add extract_ip function to handle client IP extraction - ([d7ba7c4](https://github.com/JQiue/waline-rs/commit/d7ba7c44a0b34095d1a8bfe0813a9335f1942f60))
- Update comment handling to include client IP - ([a190933](https://github.com/JQiue/waline-rs/commit/a19093348dc63f334fa8ce6fccfd36beabbdcd09))
- Add avatar and password handling to user profile - ([f36e940](https://github.com/JQiue/waline-rs/commit/f36e940c2a1efa8b118481cf8fec912ab8c2b8c3))
- Use custom bcrypt hashing for password - ([66564e4](https://github.com/JQiue/waline-rs/commit/66564e46207daaf66c30f1174009336dbb313608))
- Implement user type setting with admin verification - ([6f8bb8d](https://github.com/JQiue/waline-rs/commit/6f8bb8d44e81d08537546aa4fb59acf9fd4773f9))

### 🐛 Bug Fixes

- Add user page route to ui handler fix not found - ([8a9e149](https://github.com/JQiue/waline-rs/commit/8a9e149f1696524aa1a74e049ea52ac74766195c))
- Secure user registration with bcrypt and add email verification - ([7543492](https://github.com/JQiue/waline-rs/commit/7543492e499254cee16b908607e1db714b194a7a))

### 🚜 Refactor

- Standardize user info response format - ([58889df](https://github.com/JQiue/waline-rs/commit/58889df5e18e5e9b0742a28d35b4f9504091d9fb))

### 📚 Documentation

- Update README and config for default host value - ([84af954](https://github.com/JQiue/waline-rs/commit/84af954b01d2f9f909b395690b9d42274912caff))

### ⚙️ Miscellaneous Tasks

- Update leancloud.sh to copy waline-mini from new path - ([3cb32d9](https://github.com/JQiue/waline-rs/commit/3cb32d9106de94d8f4524652fa5a9d7cb6955df6))
- Release waline-mini version 0.8.0 - ([41765fc](https://github.com/JQiue/waline-rs/commit/41765fc36ef0609aee4f47396bdce9eb9cfd42ac))

## ❤️ New Contributors

* @JQiue made their first contribution
## [0.7.1](https://github.com/JQiue/waline-rs/compare/v0.7.0..v0.7.1) - 2025-01-04

### 🐛 Bug Fixes

- Fixed data import errors caused by fields - ([ee8242f](https://github.com/JQiue/waline-rs/commit/ee8242f70cc8f774f864a694b963073b20d84012))

### ⚙️ Miscellaneous Tasks

- Update changelog body - ([dac8bd8](https://github.com/JQiue/waline-rs/commit/dac8bd8e40551e95df0a4811af5f22c588344dbf))
- Update release.push to true - ([610d57e](https://github.com/JQiue/waline-rs/commit/610d57e9e560dd4f86d5b881da1f590057e02d3c))
- Release waline-mini version 0.7.1 - ([a23344d](https://github.com/JQiue/waline-rs/commit/a23344d592db46af0a5c57447f5f4b1bbf5ef965))

## [0.7.0](https://github.com/JQiue/waline-rs/compare/v0.6.1..v0.7.0) - 2025-01-03

### 🚀 Features

- Impl mail vertification for registered users - ([e4184ba](https://github.com/JQiue/waline-rs/commit/e4184ba2fa0ba6475354d55e403daa6641f60baa))
- Impl data migration - ([991ce7b](https://github.com/JQiue/waline-rs/commit/991ce7bd0540f8e363cc6c6acffc21b096851581))
- Add traits to print Result Err enum - ([1f64ac7](https://github.com/JQiue/waline-rs/commit/1f64ac71ab9ee6891ac2ac8f6519d17925c953d2))
- Use multiavatar as the default avatar - ([bdce4e1](https://github.com/JQiue/waline-rs/commit/bdce4e124ff3995e23e0be84c200c570b21b9796))
- Add redirect processing to the ui - ([f4aecd4](https://github.com/JQiue/waline-rs/commit/f4aecd4400eb50084a0db56eeb6604761f2fd6f4))

### 🐛 Bug Fixes

- Fixed time formatting when importing data and inability to import comments - ([4990dba](https://github.com/JQiue/waline-rs/commit/4990dba35c1a70218fe21832574aaa9ca0235bd8))
- Fix migrate data error - ([5071c9b](https://github.com/JQiue/waline-rs/commit/5071c9b822a62525c74ded229338f03bb830b51b))
- Fixed date and time handling when data is migrated - ([bab6573](https://github.com/JQiue/waline-rs/commit/bab6573cfdce8a7ca93d26950cad2ee76524e54c))

### 🚜 Refactor

- Rewrite code - ([b984a94](https://github.com/JQiue/waline-rs/commit/b984a94e531205dc01f586764eac2966941acdb0))
- Rewrite use statement - ([4eeb281](https://github.com/JQiue/waline-rs/commit/4eeb2811dec40735389d853c017b20b5dc64ab7d))

### ⚙️ Miscellaneous Tasks

- Fix leancloud health check - ([3d65a3a](https://github.com/JQiue/waline-rs/commit/3d65a3ad839a720ba8e66daaf6e7a3b499ef38fc))
- Update release.yml - ([4f30b7b](https://github.com/JQiue/waline-rs/commit/4f30b7b8e5113090a3a679a310e9d2e47e90f1a7))
- Update .gitignore - ([5e1d69b](https://github.com/JQiue/waline-rs/commit/5e1d69bcf6246c333cc9b315c5f81e961e90e911))
- Assigned copyright - ([eddf97b](https://github.com/JQiue/waline-rs/commit/eddf97b93498face1bfcf5e832617259c36baacf))
- Update .gitignore - ([e50007c](https://github.com/JQiue/waline-rs/commit/e50007c82d4670df6d5ca3b4e2a2091d608da90e))
- Adjust program internal configuration variables - ([e5980f0](https://github.com/JQiue/waline-rs/commit/e5980f0f352f1ffd087f1356098b162da7af2b9f))
- Change how to get the profile picture - ([fabbda7](https://github.com/JQiue/waline-rs/commit/fabbda760002376db810e9b99c64a82a283758b1))
- Update README.md - ([29927f0](https://github.com/JQiue/waline-rs/commit/29927f032df60621a98c944c82e44e6ec5974fae))
- Release waline-mini version 0.7.0 - ([7c90d3a](https://github.com/JQiue/waline-rs/commit/7c90d3a4a564f753bccb7dd464ae752169379d46))

## [0.6.1](https://github.com/JQiue/waline-rs/compare/v0.5.0..v0.6.1) - 2024-12-22

### 🚀 Features

- Impl the frequency limit of comment posting - ([c79fa8e](https://github.com/JQiue/waline-rs/commit/c79fa8e6f520400f8b6d7fc7cfc7cb023cde6608))
- Impl comment review - ([a0b055a](https://github.com/JQiue/waline-rs/commit/a0b055aabca802c902a0eb4455eb94468cfc63c9))
- Impl prevent flooding - ([2d72eea](https://github.com/JQiue/waline-rs/commit/2d72eea4fa4f3ca4a67356b06f7d56512f6a8726))

### ⚡ Performance

- Improves the response time to creating comments - ([9a4ef92](https://github.com/JQiue/waline-rs/commit/9a4ef9276e033f04b5d6d4f54894e8a1e7f2b513))

### ⚙️ Miscellaneous Tasks

- Update README.md - ([4a45f4c](https://github.com/JQiue/waline-rs/commit/4a45f4cdc490ffe8eda6a752f3bc5f193d8e82c1))
- Enhance release workflow for multiple build targets - ([fa32e0d](https://github.com/JQiue/waline-rs/commit/fa32e0deb9bc67191d5cfa19c0bc06a48c1195e2))
- Release v0.6.0 - ([7151266](https://github.com/JQiue/waline-rs/commit/7151266ab36d8ef84d0a7e33749a932f2e349ef5))
- Specify CHANGELGO.md - ([89b5f81](https://github.com/JQiue/waline-rs/commit/89b5f8146b72e9b4a5ccf988b6566444a65a8345))
- Use rustls to solve musl compilation issues and add LeanCloud feature for targeted deployment - ([304deed](https://github.com/JQiue/waline-rs/commit/304deedd57b9439dd4ce5df1c3da46624b5783cb))
- Release v0.6.1 - ([afb26f6](https://github.com/JQiue/waline-rs/commit/afb26f61f42b51c84ccf7c7fe1286ab71df58178))
- Change to rustls for lettre - ([be2bded](https://github.com/JQiue/waline-rs/commit/be2bded72853c2ac83b8242617e29f7cf35f6ed0))
- Update ci - ([a0da3cb](https://github.com/JQiue/waline-rs/commit/a0da3cb1c4c47f696f5113929a2efc12e8962662))
- Disable default features for lettre - ([9852037](https://github.com/JQiue/waline-rs/commit/9852037ed37d3a36c93d7fe6687708f0a6766d2a))
- Fix ci - ([4f24d15](https://github.com/JQiue/waline-rs/commit/4f24d15bb9eb7b60d4acb58dd88c658b27242a88))

## [0.5.0](https://github.com/JQiue/waline-rs/compare/v0.4.2..v0.5.0) - 2024-12-20

### 🚀 Features

- Refactor comment and user error handling - ([b400753](https://github.com/JQiue/waline-rs/commit/b400753d5dbbe3a901cee3a728ab70188d968436))
- Impl site notifications when a new comment is added - ([d8c348c](https://github.com/JQiue/waline-rs/commit/d8c348c34caf79de1b7c9a90be015d6645637768))
- Localization of in-station notifications - ([24d56ac](https://github.com/JQiue/waline-rs/commit/24d56ac1e705b6d09624053e01f7b169deac914f))
- Add custom user grade levels support - ([5dd0a8d](https://github.com/JQiue/waline-rs/commit/5dd0a8de7cd609be75f4c9da9bc7299d9b644f48))
- Add Ammonia HTML sanitizer to prevent XSS - ([21e7e0f](https://github.com/JQiue/waline-rs/commit/21e7e0f9201415605f1f1a7c30a9b82c81d86833))

### ⚙️ Miscellaneous Tasks

- Update READEME.md - ([9b8e08c](https://github.com/JQiue/waline-rs/commit/9b8e08c5828216419911e7c876e3f20ea625ee5e))
- Update README.md - ([c8fb4ad](https://github.com/JQiue/waline-rs/commit/c8fb4ad1e05919fdb67ad2a06bbfb206820768db))
- Commit LeanCloud deployment - ([f327ce7](https://github.com/JQiue/waline-rs/commit/f327ce78474affaf381af3297cf6ac93c22c12da))
- Update README.md - ([46fbd26](https://github.com/JQiue/waline-rs/commit/46fbd26de04c1af7caf61c13e2545c5c1d4e245b))
- Update README.md - ([921c913](https://github.com/JQiue/waline-rs/commit/921c913a735193253d9ccaee50b120de078754b5))
- Update Cargo.lock - ([ef757b4](https://github.com/JQiue/waline-rs/commit/ef757b415ab44ae11a66c5aa5898cd7009f15a40))
- Update READEME.md - ([fe94f4f](https://github.com/JQiue/waline-rs/commit/fe94f4f66fdfa740f39d19d04b5359f34976c733))
- Release waline-mini version v0.5.0 - ([8b0bc67](https://github.com/JQiue/waline-rs/commit/8b0bc67f16875c15b0e003163958994a0f83fb5b))

## [0.4.2](https://github.com/JQiue/waline-rs/compare/v0.4.1..v0.4.2) - 2024-12-11

### 🐛 Bug Fixes

- Fixed counter time can't update - ([add3881](https://github.com/JQiue/waline-rs/commit/add3881ab87063ecdadd5d9690ca543bb07c29da))
- Fixed can't create momment error - ([65e933d](https://github.com/JQiue/waline-rs/commit/65e933da3d33e37a535a7927cff13723a1ddfd92))
- Fixed can't set user profile error - ([8ef0be2](https://github.com/JQiue/waline-rs/commit/8ef0be286a5cfd77b12598098a72264e077242c7))

### ⚙️ Miscellaneous Tasks

- Release waline-mini version 0.4.2 - ([36afb2d](https://github.com/JQiue/waline-rs/commit/36afb2d59c75ee16945b2e7b984924f0b6f73a6f))

## [0.4.1](https://github.com/JQiue/waline-rs/compare/v0.4.0..v0.4.1) - 2024-12-10

### 🐛 Bug Fixes

- Fixed an issue where counters could not be created - ([16b3309](https://github.com/JQiue/waline-rs/commit/16b3309b3ff06c1f6edea3563bf87d6fd5596ed1))

### ⚙️ Miscellaneous Tasks

- Release walini-mini version v0.4.1 - ([2ae8289](https://github.com/JQiue/waline-rs/commit/2ae8289bef87f0012dfad2f8cc230f7625e0a63d))

## [0.4.0](https://github.com/JQiue/waline-rs/compare/v0.3.0..v0.4.0) - 2024-12-10

### 🚀 Features

- Add sqlite driver - ([b41295d](https://github.com/JQiue/waline-rs/commit/b41295d6d5f29446cc99f4e9fb9894ddee6bfbac))

### ⚙️ Miscellaneous Tasks

- Release waline-mini version 0.4.0 - ([9e8e86c](https://github.com/JQiue/waline-rs/commit/9e8e86c83fc0bb0ee74a5ad8b6321bc6133e1072))

## [0.3.0](https://github.com/JQiue/waline-rs/compare/v0.2.0..v0.3.0) - 2024-12-10

### 🚀 Features

- *(error)* Implement more robust error mapping - ([01b278b](https://github.com/JQiue/waline-rs/commit/01b278bd55250f17e3f07a5e85152dfb83f74906))

### 🚜 Refactor

- Simplify create_comment_data error handling - ([67465aa](https://github.com/JQiue/waline-rs/commit/67465aa29a9c0921b84f53c0937e7f70d6ff1ba4))

### ⚙️ Miscellaneous Tasks

- Supplement metedata and add the CHANGELOG.md generation tool - ([134473c](https://github.com/JQiue/waline-rs/commit/134473c75c83b9af546b1d0441d6e04f16d1af7f))
- Release waline-mini version 0.3.0 - ([0653531](https://github.com/JQiue/waline-rs/commit/0653531e231fbaa93c5cb70552e0c7f3fb6c5bc9))

## [0.2.0] - 2024-09-22

### 🚀 Features

- Implement most api - ([e5a6fa1](https://github.com/JQiue/waline-rs/commit/e5a6fa1cf1230e18853a5b90697b2ed5ba3bd687))
- Rename project to waline-mini and update readme - ([5dc9533](https://github.com/JQiue/waline-rs/commit/5dc9533a099147989fdc677a549e7fda9195c396))
- Add functionality to update comment - ([37862ff](https://github.com/JQiue/waline-rs/commit/37862ff14eed42dfb1f415caf656fab0433e37c5))
- Add functionality to register user - ([83a0a49](https://github.com/JQiue/waline-rs/commit/83a0a498806f8c0466f0bf68412cdab387f05ec1))
- Refactor: add i18n support for user messages - ([1abf7c2](https://github.com/JQiue/waline-rs/commit/1abf7c2dcbfc545c49266c20e5d8a9c7f3c17e6b))

### 🐛 Bug Fixes

- Update token expiration logic - ([e6e6137](https://github.com/JQiue/waline-rs/commit/e6e61370d6f868ed088e960924360c3acaf3c575))
- Fixed bug in update_article function - ([bacb988](https://github.com/JQiue/waline-rs/commit/bacb988d88cc15cb15c57e0296fb3b7d21047fc5))

### 🚜 Refactor

- Refactor build_data_entry function for clarity - ([408b9e6](https://github.com/JQiue/waline-rs/commit/408b9e651a568b029d22ef0168e8ae1cb8a31f1d))
- Extract create_comment_model function - ([0a75d79](https://github.com/JQiue/waline-rs/commit/0a75d7946d45e27bfc2de4ff1bb38e8d3852b4f9))
- Apply lint fixes to the project - ([d347187](https://github.com/JQiue/waline-rs/commit/d3471875b11d2b783385fceea1715874b4c5efeb))
- Improve code logic in get_article and update_article endpoints - ([aed58b5](https://github.com/JQiue/waline-rs/commit/aed58b5c3f0ebb1a4675cc1881b4741f9202f0b5))
- Restructure project with component architecture - ([30df5af](https://github.com/JQiue/waline-rs/commit/30df5af758224396df4ac9fbfc071dd634fe87ab))

### ⚙️ Miscellaneous Tasks

- Update multiple dependencies - ([1825540](https://github.com/JQiue/waline-rs/commit/1825540137e7e88d2a6f22e4d2b174a2bce9b617))
- Add Github Workflows for release - ([c588593](https://github.com/JQiue/waline-rs/commit/c588593a7d95dea1638487ced93f4dc736e74f98))
- Fix workflow - ([8faf2b9](https://github.com/JQiue/waline-rs/commit/8faf2b9b1157e21773b358021dbb20208190cf0e))

## ❤️ New Contributors

* @JQiue made their first contribution

