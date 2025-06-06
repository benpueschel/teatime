# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.1](https://github.com/benpueschel/teatime/compare/0.5.0..0.5.1) - 2025-04-28

### 🐛 Bug Fixes

- Gh action cache migration

### Build

- *(deps)* Bump tokio in the cargo group across 1 directory (#17)
- *(deps)* Bump tokio in the cargo group across 1 directory (#18)

<!-- generated by git-cliff -->
## [0.5.0](https://github.com/benpueschel/teatime/compare/0.4.1..0.5.0) - 2025-01-12

### 💥 BREAKING CHANGES

- due to [9d83db7](https://github.com/benpueschel/teatime/commit/9d83db74ec64f419d19b7b2f8a885cba8c364e8b) - add option to the pulls model (#9):
  add option to the pulls model (#9)


### 🚀 Features

- Add request get reviews (#10)

### 🐛 Bug Fixes

- [**breaking**] Add option to the pulls model (#9)

### ⚙️ Miscellaneous Tasks

- Cargo update

<!-- generated by git-cliff -->
## [0.4.1](https://github.com/benpueschel/teatime/compare/0.4.0..0.4.1) - 2024-10-06

### 🚀 Features

- Issue comments

### 🐛 Bug Fixes

- *(comments)* Broken gitea api behavior

### ⚙️ Miscellaneous Tasks

- Add license
- Remove unused dependency

<!-- generated by git-cliff -->
## [0.4.0](https://github.com/benpueschel/teatime/compare/0.3.0..0.4.0) - 2024-09-10

### 🚀 Features

- Repo branch operations
- List a users organizations
- Create and list an organization's repos
- Manage an organization's members
- Manage an organization's public members

### 🐛 Bug Fixes

- *(repos)* Add external_tracker and external_wiki ([#6](https://github.com/benpueschel/teatime/pull/6))

<!-- generated by git-cliff -->
## [0.3.0](https://github.com/benpueschel/teatime/compare/0.2.1..0.3.0) - 2024-09-07

### 💥 BREAKING CHANGES

- due to [1d8c04f](https://github.com/benpueschel/teatime/commit/1d8c04fbee9173bfe5f91cb157ad427f09df5579) - move method to get a user into `users`:
  `client.user().get("username")` will now be
  `client.users("username").get()`


### 🚀 Features

- Add basic pull request support
- Use impl traits in builder structs
- Get and update current user's settings
- Manage user's starred repos
- Get a user's starred repos by username
- Get a user's repos by username

### 🚜 Refactor

- Remove custom proc-macro
- [**breaking**] Move method to get a user into `users`

### 🧪 Testing

- Fix edit pr test

### ⚙️ Miscellaneous Tasks

- Cargo update

<!-- generated by git-cliff -->
## [0.2.1](https://github.com/benpueschel/teatime/compare/0.2.0..0.2.1) - 2024-09-05

### 🚀 Features

- Basic organization support
- Get current user's orgs
- List the current user's access tokens

### 🐛 Bug Fixes

- *(org)* Add tests and make them pass (oops)

### 🧪 Testing

- *(org)* Add integration tests

### ⚙️ Miscellaneous Tasks

- Release-plz workflow
- Pretty changelogs :)
- *(rust)* Only run if code changed

### Build

- *(deps)* Bump quinn-proto in the cargo group across 1 directory

<!-- generated by git-cliff -->
