# calendar

イベント情報を管理していくカレンダーアプリ

## Development

### Getting started

開発環境は Docker を利用することを想定しています。推奨する Docker ランタイムは Colima です。

```
make setup
docker compose up -d
```

起動すると [http://localhost:8081/] でアクセスできます。

* API の起動に若干時間がかかるので最初はカレンダーの内容が表示されない場合があります

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
