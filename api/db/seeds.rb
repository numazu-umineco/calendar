ApplicationRecord.transaction do
  cal = Calendar::Detail.create!(id: 'numazu-city', name: '沼津市イベントカレンダー')

  cal.events.create!(
    summary: '沼津夏まつり・狩野川花火大会 #1',
    description: '沼津夏まつり・狩野川花火大会',
    start_at: DateTime.parse('2023-07-29 17:00'),
    end_at: DateTime.parse('2023-07-29 20:00'),
    location: '沼津市狩野川河川敷',
    last_modified_user: 'umineco-admin',
  )
  cal.events.create!(
    summary: '沼津夏まつり・狩野川花火大会 #2',
    description: '沼津夏まつり・狩野川花火大会',
    start_at: DateTime.parse('2023-07-30 17:00'),
    end_at: DateTime.parse('2023-07-30 20:00'),
    location: '沼津市狩野川河川敷',
    last_modified_user: 'umineco-admin',
  )
  cal.events.create!(
    summary: 'ラブライブ！サンシャイン!!沼津地元愛まつり 2023  ',
    description: '＜Day.1＞
2023年10月7日(土)
＜昼公演＞13：00開場／14：00開演
＜夜公演＞17：30開場／18：30開演
【出演】伊波杏樹（高海千歌／チカ役）、高槻かなこ（国木田花丸／ハナマル役）、鈴木愛奈（小原鞠莉／マリ役）
',
    start_at: DateTime.parse('2023-10-07 13:00'),
    end_at: DateTime.parse('2023-10-07 20:00'),
    location: 'キラメッセぬまづ',
    last_modified_user: 'umineco-admin',
  )
  cal.events.create!(
    summary: 'ラブライブ！サンシャイン!!沼津地元愛まつり 2023  ',
    description: '＜Day.2＞
2023年10月8日(日)
＜昼公演＞13：00開場／14：00開演
＜夜公演＞17：30開場／18：30開演
【出演】諏訪ななか（松浦果南／カナン役）、小宮有紗（黒澤ダイヤ／ダイヤ役）、斉藤朱夏（渡辺 曜／ヨウ役）

',
    start_at: DateTime.parse('2023-10-08 13:00'),
    end_at: DateTime.parse('2023-10-08 20:00'),
    location: 'キラメッセぬまづ',
    last_modified_user: 'umineco-admin',
  )
  cal.events.create!(
    summary: 'ラブライブ！サンシャイン!!沼津地元愛まつり 2023  ',
    description: '＜Day.3＞
2023年10月9日(月・祝)
＜昼公演＞13：00開場／14：00開演
＜夜公演＞17：30開場／18：30開演
【出演】逢田梨香子（桜内梨子／リコ役）、小林愛香（津島善子／ヨハネ役）、降幡 愛（黒澤ルビィ／ルビィ役）
',
    start_at: DateTime.parse('2023-10-09 13:00'),
    end_at: DateTime.parse('2023-10-09 20:00'),
    location: 'キラメッセぬまづ',
    last_modified_user: 'umineco-admin',
  )
end
