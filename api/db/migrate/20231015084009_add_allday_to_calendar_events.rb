class AddAlldayToCalendarEvents < ActiveRecord::Migration[7.1]
  def change
    add_column :calendar_events, :all_day, :boolean, default: false, null: false
  end
end
