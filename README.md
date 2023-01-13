# bkp

simple backup utility

file level deduplication

backups are saved locally, and remotely via s3

for a quick to run s3 backend try minio, other alternatives are also available. bkp expects to have a `bkp` bucket created in s3.

## how to build

To run, you need rust, (install with rustup) Then

```
cargo build --release
```

## how to run

first you need a .bkpconfig file in your homedir, see example dir for content

then create a config file for each app you want to backup, see example/config dir

then run `bkp` manually, or schedule via cron

## backup configuration

backups are configured via toml files, example:

```
app_name = 'app1'
app_root = '/Users/ondrej/testdata/app1/'

included_paths = ['**/*']
excluded_paths = ['logs.txt']

pre_backup_script = '/Users/ondrej/testdata/scripts/app1/pre_backup.sh'
post_backup_script = '/Users/ondrej/testdata/scripts/app1/post_backup.sh'

pre_restore_script = ''
post_restore_script = ''

incremental_backup_interval_days = 1
full_backup_periods = [7, 30, 180]
backup_start_time = '04:10'

local_storage_location = '/Users/ondrej/testdata/storage'

remote_storage_address = 'user@server'
remote_location = '/home/backup'
```

## cli usage

```
Usage: bkp [COMMAND]

Commands:
  list     Lists all backups
  backup   Backs apps up according to config file
  restore  Restores an app from a specific backup
  help     Print this message or the help of the given subcommand(s)
```
