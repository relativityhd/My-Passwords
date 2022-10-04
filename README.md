# My-Passwords

My password generator. Made with ♥ and Tauri

## Dev notice

Start Dev-DB in Docker on port `3306` (ensure that the directory `/tmp/my-passwords/mysql` exists)

```sh
docker run --detach --name my-passwords -v /tmp/my-passwords/mysql:/var/lib/mysql -p 3306:3306 --env MARIADB_USER=dev --env MARIADB_PASSWORD=dev --env MARIADB_ROOT_PASSWORD=root --env MARIADB_DATABASE=mypasswords  mariadb:latest
```

Connect via

```sh
mysql -h 127.0.0.1 -P 3306 -u root -p
```
