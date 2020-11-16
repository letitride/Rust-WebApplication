dieselを使用したpostgresqlのsetup
```
$ cargo install diesel_cli --no-default-features --features postgres
```

create database (.evnを導入した場合は、--database-urlは.envに記述できる)
```
$ cd server && diesel setup  --database-url postgresql://postgres:password@localhost:5432/log-collector
```

migration
```
$ diesel migration run --database-url postgresql://postgres:password@localhost:5432/log-collector
```


##### Command

post: json data
```
$ curl -v -H 'Content-Type: application/json' -d '{"user_agent":"Mozilla", "response_time":200}' localhost:3000/logs
```

post: csv mutipart 
```
$ curl -F 'file=@test.csv;type=text/csv' http://localhost:3000/csv
```

get: csv
```
curl http://localhost:3000/csv
```


マイグレートファイルの作成
```
$ diesel migration generate create_logs
```