# calendar

イベント情報を管理していくカレンダーアプリ

## Development

### Getting started

開発環境は Docker を利用することを想定しています。推奨する Docker ランタイムは Colima です。

```bash
make setup
docker compose up -d
```

起動すると [http://localhost:8081/] でアクセスできます。

API の起動に若干時間がかかるので最初はカレンダーの内容が表示されない場合があります。

### Litestream について

データベースには SQLite + Litestream を使ってレプリケーションを行っています。

開発環境では MinIO コンテナ上にデータがホストされることを想定し、 API コンテナの立ち上げ時に自動的にリストア・レプリケーションされるようにしてあります。

API (Rails) でのテストの実行など、 Litestream が不要な環境については、環境変数 `RAILS_ENV=test` または `DISABLE_LITESTREAM=on` を指定してください。

```bash
# Run minitest
docker compose run --rm -e RAILS_ENV=test api bundle exec rails test
# Run Rubocop
docker compose run --rm -e DISABLE_LITESTREAM=on api bundle exec rubocop
```

### ディレクトリ構成

```
├── admin   ....... イベント登録管理者用フロントエンド
├── api     ....... API サーバー (Rails)
└── website ....... 一般公開用ウェブページ
```

## License

[MIT License](LICENSE)

## Copyright

&copy; 2023 umineco
