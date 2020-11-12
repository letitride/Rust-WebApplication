dieselを使用したpostgresqlのsetup
```
$ cargo install diesel_cli --no-default-features --features postgres
```

create database
```
$ cd server && diesel setup  --database-url postgresql://postgres:password@localhost:5432/log-collector
```

migration
```
$ diesel migration run --database-url postgresql://postgres:password@localhost:5432/log-collector
```


マイグレートファイルの作成
```
$ diesel migration generate create_logs
```