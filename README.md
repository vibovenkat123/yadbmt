## To hash password

To figure out your hashed password for connecting, run this script

```
$ python3 -c 'import urllib.parse;print(urllib.parse.quote(input("Password: "),""))'
Password: 123#
123%23
```
