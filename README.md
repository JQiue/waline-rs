<div align="center">
 <p><h1>waline-mini</h1></p>
  <p><strong>A minimalist implementation of waline.</strong></p>
  <p>

![GitHub Release](https://img.shields.io/github/v/release/JQiue/waline-mini)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/JQiue/waline-mini)
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/JQiue/waline-mini)
[![Version](https://img.shields.io/badge/rustc-1.75+-lightgray.svg)](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/JQiue/waline-mini/total)
![GitHub License](https://img.shields.io/github/license/JQiue/waline-mini)
  </p>
</div>

## Introduction

Waline-mini is a lightweight Rust implementation of the Waline comment system, using 95% less memory than its Node.js counterpart and serving as an efficient alternative for resource-constrained servers.

In my Ubuntu server, the waline-mini requires only about `5612Kb=5.48MB` of memory

![mem](./assets/image.png)

+ Extremely low memory usage: Just 1/25 of the Node.js version's memory footprint.
+ Seamless replacement: Drop-in alternative to the original Waline.
+ Synchronized development: Keeping pace with the original Waline's evolution.

## Features

| Feature                    | Availability     | Status      |
| -------------------------- | ---------------- | ----------- |
| Pageview Counter           | Fully Available  | Stable      |
| Article Reactions          | Fully Available  | Stable      |
| Comment Format Support     | Fully Available  | Stable      |
| User Label                 | Fully Available  | Stable      |
| I18n Support               | Nearly Available | In Progress |
| Email Notification         | Nearly Available | In Progress |
| Security: XSS              | Fully Available  | Stable      |
| Security: Frequency Limit  | Fully Available  | Stable      |
| Security: Prevent flooding | Fully Available  | Stable      |
| Security: Comment Review   | Fully Available  | Stable      |
| Data migration             | Fully Available  | Stable      |

## Usage

From [GitHub Releases](https://github.com/JQiue/waline-mini/releases) to download the binary file is appropriate for your platform.

```bash
# Setting environment variables
export DATABASE_URL=sqlite:///path/to/waline.sqlite
export JWT_TOKEN=your_secret_key
export SITE_NAME=your_site_name
export SITE_URL=your_site_url

# Start
./waline-mini
```

Configure waline-mini with environment variables:

| Environment variable | Description                                                                                                                                               | Require | Default     |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- | ----------- |
| DATABASE_URL         | SQLite and MySQL/MariaDB are supported. `protocol://username:password@host/database`                                                                      | ✅       | -           |
| JWT_TOKEN            | A random string is used to generate the JWT Signature key                                                                                                 | ✅       | -           |
| SITE_NAME            | Site name                                                                                                                                                 | ✅       | -           |
| SITE_URL             | Site url                                                                                                                                                  | ✅       | -           |
| SERVER_URL           | Custom Waline server address                                                                                                                              |         | auto        |
| WORKERS              | Worker thread count                                                                                                                                       |         | 1           |
| LEVELS               | Give each user a rating label based on the number of comments                                                                                             |         | -           |
| SMTP_SERVICE         | SMTP mail service provider                                                                                                                                |         | -           |
| SMTP_HOST            | SMTP server address                                                                                                                                       |         | -           |
| SMTP_PORT            | SMTP server port                                                                                                                                          |         | -           |
| SMTP_USER            | SMTP username                                                                                                                                             |         | -           |
| SMTP_PASS            | SMTP Password                                                                                                                                             |         | -           |
| AUTHOR_EMAIL         | The blogger’s email, used to judge whether posted comment is posted by the blogger.If it is posted by the blogger, there will be no reminder notification |         | -           |
| IPQPS                | IP-based comment posting frequency limit in seconds. Set to `0` for no limit                                                                              |         | `60`        |
| COMMENT_AUDIT        | Comment audit switcher. When enabled, every comment needs to be approved by admin, so hint in placeholder is recommended                                  |         | `false`     |
| HOST                 | listening host                                                                                                                                            |         | `127.0.0.1` |
| PORT                 | listening port                                                                                                                                            |         | `8360`      |

## References

+ [waline-api](https://waline.js.org/next/api/)
