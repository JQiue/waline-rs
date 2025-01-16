<div align="center">
 <p><h1>waline-mini</h1></p>
  <p>English | <a href="./README.zh-CN.md">简体中文</a></p>
  <p><strong>A minimalist implementation of Waline.</strong></p>
  <p>

![GitHub Release](https://img.shields.io/github/v/release/JQiue/waline-mini)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/JQiue/waline-mini)
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/JQiue/waline-mini)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/JQiue/waline-mini/total)
![GitHub License](https://img.shields.io/github/license/JQiue/waline-mini)
  </p>
</div>

## Introduction

Waline-mini is a lightweight Rust implementation of the Waline comment system, using 95% less memory than its Node.js counterpart and serving as an efficient alternative for resource-constrained servers.

In my Ubuntu server, the waline-mini requires only about `5612Kb=5.48MB` of memory

![mem](./assets/image.png)

+ **Extremely low memory usage**: Just 1/25 of the Node.js version's memory footprint.
+ **Easy replacement**: Implements most of the necessary apis of the original Waline.
+ **Synchronous update**: Keeping pace with the original Waline's evolution.

## Features

| Feature                      | Availability | Status      |
| ---------------------------- | ------------ | ----------- |
| Pageview Counter             | Fully        | Stable      |
| Article Reactions            | Fully        | Stable      |
| Comment Format Support       | Fully        | Stable      |
| User Label                   | Fully        | Stable      |
| I18n Support                 | Nearly       | In Progress |
| Email Notification           | Nearly       | In Progress |
| Security: XSS                | Fully        | Stable      |
| Security: Frequency Limit    | Fully        | Stable      |
| Security: Prevent flooding   | Fully        | Stable      |
| Security: Comment Review     | Fully        | Stable      |
| Security: Anti-spam comments | Fully        | Stable      |
| Data migration               | Fully        | Stable      |

## Usage

### Run from an executable file

From [GitHub Releases](https://github.com/JQiue/waline-mini/releases) to download the binary file is appropriate for your platform. Examples of Linux use. You first need to get the `waline.sqlite` file prepared from the `assets`:

```bash
# Setting environment variables
export DATABASE_URL=sqlite:///path/to/waline.sqlite
export JWT_TOKEN=your_secret_key
export SITE_NAME=your_site_name
export SITE_URL=your_site_url

# Start
./waline-mini
```

### Docker

```sh
docker run -d \
  -e JWT_TOKEN=your_secret_key \
  -e SITE_NAME=your_site_name \
  -e SITE_URL=your_site_url \
  -p 8360:8360 \
  jqiue/waline
```

Since the image is packaged with built-in SQLite as the default storage, you do not need to specify `DATABASE_URL` when using SQLite as storage, if you want to use other databases, you only need to add `-e DATABASE_URL` environment for coverage

### Shuttle

waline-mini supports deployment on Shuttle by first cloning the `shuttle` branch to the local using the following command

```sh
git clone -b shuttle https://github.com/JQiue/waline-mini.git
```

Then, create a `.shuttle.env` environment variable in the project root to configure waline-mini

Finally, in accordance with the [Shuttle](https://console.shuttle.dev/login) steps for deployment

### LeanCloud

When LeanCloud is used to pull the warehouse directly for deployment, the branch needs to enter "leancloud"

If SQLite is used as the data store, the environment variable `DATABASE_URL` should be filled with `sqlite://./waline.sqlite? mode=rw`. When deploying with LeanCloud, a new SQLite file is included each time, so it is important to export the data before redeployment and import the data after redeployment when upgrading the waline-mini for redeployment

## Configuration

Configure waline-mini with environment variables:

| Environment variable | Description                                                                                                                                               | Require | Default        |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- | -------------- |
| DATABASE_URL         | SQLite and MySQL/MariaDB are supported. Compile features can be added to support PostgreSQL at any time. `protocol://username:password@host/database`     | ✅       | -              |
| JWT_TOKEN            | A random string is used to generate the JWT Signature key                                                                                                 | ✅       | -              |
| SITE_NAME            | Site name                                                                                                                                                 | ✅       | -              |
| SITE_URL             | Site url                                                                                                                                                  | ✅       | -              |
| SERVER_URL           | Custom Waline server address                                                                                                                              |         | auto           |
| WORKERS              | Worker thread count                                                                                                                                       |         | 1              |
| LEVELS               | Give each user a rating label based on the number of comments                                                                                             |         | -              |
| SMTP_SERVICE         | SMTP mail service provider: `QQ`，`GMail`，`126`，`163`                                                                                                   |         | -              |
| SMTP_HOST            | SMTP server address                                                                                                                                       |         | -              |
| SMTP_PORT            | SMTP server port                                                                                                                                          |         | -              |
| SMTP_USER            | SMTP username                                                                                                                                             |         | -              |
| SMTP_PASS            | SMTP Password                                                                                                                                             |         | -              |
| AUTHOR_EMAIL         | The blogger’s email, used to judge whether posted comment is posted by the blogger.If it is posted by the blogger, there will be no reminder notification |         | -              |
| IPQPS                | IP-based comment posting frequency limit in seconds. Set to `0` for no limit                                                                              |         | `60`           |
| COMMENT_AUDIT        | Comment audit switcher. When enabled, every comment needs to be approved by admin, so hint in placeholder is recommended                                  |         | `false`        |
| AKISMET_KEY          | Akismet antispam service key, set `false` if you wanna close it.                                                                                          |         | `86fe49f5ea50` |
| LOGIN                | User need login before comment when `LOGIN=force`                                                                                                         |         | `false`        |
| HOST                 | listening host                                                                                                                                            |         | `127.0.0.1`    |
| PORT                 | listening port                                                                                                                                            |         | `8360`         |

## References

+ [waline-api](https://waline.js.org/next/api/)
