# YADBMT (Yet another database migration tool)

This is a tool you can use for managing migrations on a postgres database

You need to set the env variable `DB_URL` which should be the postgres url for connecting

## To hash password

To figure out your hashed password for connecting, run this script

```
$ python3 -c 'import urllib.parse;print(urllib.parse.quote(input("Password: "),""))'
Password: 123#
123%23
```
