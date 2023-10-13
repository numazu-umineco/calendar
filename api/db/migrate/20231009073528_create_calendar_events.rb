class CreateCalendarEvents < ActiveRecord::Migration[7.1]
  def change
    create_table :calendar_events do |t|
      t.text :summary, null: false
      t.text :description, null: false
      t.text :location
      t.float :latitude
      t.float :longitude
      t.datetime :start_at, null: false
      t.datetime :end_at, null: false
      t.datetime :discarded_at
      t.references :calendar_detail, null: false, foreign_key: true, type: :string
      t.string :last_modified_user, null: false, default: ''

      t.timestamps

      t.index :discarded_at
    end
  end
end
