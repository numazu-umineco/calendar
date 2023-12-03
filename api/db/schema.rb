# This file is auto-generated from the current state of the database. Instead
# of editing this file, please use the migrations feature of Active Record to
# incrementally modify your database, and then regenerate this schema definition.
#
# This file is the source Rails uses to define your schema when running `bin/rails
# db:schema:load`. When creating a new database, `bin/rails db:schema:load` tends to
# be faster and is potentially less error prone than running all of your
# migrations from scratch. Old migrations may fail to apply correctly if those
# migrations use external dependencies or application code.
#
# It's strongly recommended that you check this file into your version control system.

ActiveRecord::Schema[7.1].define(version: 2023_10_15_084009) do
  create_table "calendar_details", id: :string, force: :cascade do |t|
    t.string "name", null: false
    t.datetime "discarded_at"
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.index ["discarded_at"], name: "index_calendar_details_on_discarded_at"
    t.index ["name"], name: "index_calendar_details_on_name", unique: true
  end

  create_table "calendar_events", force: :cascade do |t|
    t.text "summary", null: false
    t.text "description", null: false
    t.text "location"
    t.float "latitude"
    t.float "longitude"
    t.datetime "start_at", null: false
    t.datetime "end_at", null: false
    t.datetime "discarded_at"
    t.string "calendar_detail_id", null: false
    t.string "last_modified_user", default: "", null: false
    t.datetime "created_at", null: false
    t.datetime "updated_at", null: false
    t.boolean "all_day", default: false, null: false
    t.index ["calendar_detail_id"], name: "index_calendar_events_on_calendar_detail_id"
    t.index ["discarded_at"], name: "index_calendar_events_on_discarded_at"
  end

  add_foreign_key "calendar_events", "calendar_details"
end
