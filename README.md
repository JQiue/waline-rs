# waline-mini

> A lightweight Waline comment system implemented in Rust

## Introduction

waline-mini is a Rust implementation of the Waline comment system. Compared to the Node.js version, it only uses 1/10 of the memory, making it ideal for environments with limited server memory.

Although the functionality is not yet complete, the goal of waline-mini is not to completely replace the original Waline, but to provide a more lightweight and efficient alternative.

In my Ubuntu server, the waline-mini requires only about `5612Kb=5.48MB` of memory

![mem](./assets/image.png)

+ Extremely low memory usage: Uses only 1/10 of the memory compared to the Node.js version.
+ Fast response: Built with Rust, known for its excellent performance and low-level efficiency.
+ Easy to use: Offers similar APIs to the original Waline, making it easy to integrate and use.
+ Continual updates: Will be updated regularly to improve functionality and keep up with the development of Waline.

## Usage

From [GitHub Releases](https://github.com/JQiue/waline-mini/releases) to download the binary file is appropriate for your platform.

```bash
# Setting environment variables
export HOST=127.0.0.1
export PORT=8360
export DATABASE_URL=sqlite:///path/to/waline.sqlite
export JWT_KEY=your_secret_key

# Start
./waline-mini
```

Configure waline-mini with environment variables:

| Environment variable | Description                                                                                                                                                | Require | Default |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- | ------- |
| HOST                 | listening host                                                                                                                                             | ✅       | -       |
| PORT                 | listening port                                                                                                                                             | ✅       | -       |
| DATABASE_URL         | SQLite and MySQL/MariaDB are supported. `protocol://username:password@host/database`                                                                       | ✅       | -       |
| JWT_KEY              | A random string is used to generate the JWT Signature key                                                                                                  | ✅       | -       |
| SITE_NAME            | Site name                                                                                                                                                  | ✅       | -       |
| SITE_URL             | Site url                                                                                                                                                   | ✅       | -       |
| SERVER_URL           | Custom Waline server address                                                                                                                               |         | auto    |
| WORKERS              | Worker thread count                                                                                                                                        |         | 1       |
| LEVELS               | Give each user a rating label based on the number of comments                                                                                              |         |         |
| SMTP_SERVICE         | SMTP mail service provider                                                                                                                                 |         |         |
| SMTP_HOST            | SMTP server address                                                                                                                                        |         |         |
| SMTP_PORT            | SMTP server port                                                                                                                                           |         |         |
| SMTP_USER            | SMTP username                                                                                                                                              |         |         |
| SMTP_PASS            | SMTP Password                                                                                                                                              |         |         |
| AUTHOR_EMAIL         | The blogger’s email, used to judge whether posted comment is posted by the blogger.If it is posted by the blogger, there will be no reminder notification. |         |         |

## Features

| Feature                    | Availability     | Status      |
| -------------------------- | ---------------- | ----------- |
| Pageview Counter           | Fully Available  | Stable      |
| Article Reactions          | Fully Available  | Stable      |
| Comment Format Support     | Fully Available  | Stable      |
| User label                 | Fully Available  | Stable      |
| I18n Support               | Nearly Available | In Progress |
| Email Notification         | Nearly Available | In Progress |
| Security: XSS              | Fully Available  | Stable      |
| Security: Frequency Limit  | Not Available    | In Progress |
| Security: Prevent flooding | Not Available    | In Progress |
| Security: Comment Review   | Not Available    | In Progress |

## References

+ [waline-api](https://waline.js.org/next/api/)
