### bump-my-resume

[![Build Status](https://github.com/StackOverflowExcept1on/bump-my-resume/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/bump-my-resume/actions)

Tool for Headhunter that can automatically update resumes

Now you can simply register the application at https://dev.hh.ru/admin and get an access token once using Selenium. This
tool will simulate activity on a job search website and keep your resume on top.

### Requirements

- ChromeDriver
  - Windows: [windows-latest-chrome.ps1](https://github.com/stevepryde/thirtyfour/blob/main/ci/windows-latest-chrome.ps1)
  - Linux: [ubuntu-latest-chrome](https://github.com/stevepryde/thirtyfour/blob/main/ci/ubuntu-latest-chrome)
  - macOS: [macos-latest-chrome](https://github.com/stevepryde/thirtyfour/blob/main/ci/macos-latest-chrome)
- Account on https://hh.ru
  - `login`
  - `password`
- Registered application on https://dev.hh.ru/admin
  - `client_id` looks like `UWWE5NX5SXFVJAFVBDB3M1P8B7K3L0XK4HIJWFEPAZLW0CGRMUA997PG38I21C71`
  - `client_secret` looks like `GGIGAZEFM796C6ZSTV0O5UUNIY06GVTC45XFZKEUEEB9ZIMP2ZMICXVDGQBTF2BT`

### Installing

```bash
# run ChromeDriver on the another thread (it should use port 9515)
chromedriver &

# pass authorization, this requires a ChromeDriver
cargo build --release
# replace it with your data
./target/release/headhunter-cli auth \
  "client_id" \
  "client_secret" \
  "login" \
  "password"

# credentials are written to response.json
cat response.json

# you can now use the CLI to update the latest date on your resume
./target/release/headhunter-cli bump
```

### Configuration on server

1. put `response.json` and `./target/release/headhunter-cli` to the same directory, e.g. to `/root/bump-my-resume`
2. run `crontab -e`
3. append it with line like this: `0 */1 * * * cd /root/bump-my-resume && ./target/release/headhunter-cli bump`
