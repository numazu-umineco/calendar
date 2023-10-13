class CreateCalendarDetails < ActiveRecord::Migration[7.1]
  def change
    create_table :calendar_details, id: :string do |t|
      t.string :name, null: false
      t.datetime :discarded_at

      t.timestamps

      t.index :name, unique: true
      t.index :discarded_at
    end
  end
end
