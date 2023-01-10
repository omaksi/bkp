# bkp

simple backup solution

backups are saved locally, and remotely wi

## how to run

To run, you need rust, (install with rustup) Then

```
cargo build --release
```

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
