# == Schema Information
#
# Table name: calendar_details
#
#  id           :string           not null, primary key
#  discarded_at :datetime
#  name         :string           not null
#  created_at   :datetime         not null
#  updated_at   :datetime         not null
#
# Indexes
#
#  index_calendar_details_on_discarded_at  (discarded_at)
#  index_calendar_details_on_name          (name) UNIQUE
#
require 'test_helper'

class Calendar::DetailTest < ActiveSupport::TestCase
  should validate_presence_of(:name)
  should have_many(:events)

  test '#to_icalはイベントをまとめたical形式の文字列を返す' do
    calendar = create(:calendar_detail)
    event = create(:calendar_event, calendar: calendar, latitude: 35.0955, longitude: 138.8634)
    now = Time.zone.now
    uid = SecureRandom.uuid
    event.stubs(:gen_uuid).returns(uid)
    expects = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:icalendar-ruby\r\nCALSCALE:GREGORIAN\r\nMETHOD:PUBLISH\r\nBEGIN:VEVENT\r\nDTSTAMP:#{format_ical_datetime(now)}Z\r\nUID:#{uid}\r\nDTSTART:#{format_ical_datetime(event.start_at)}\r\nDTEND:#{format_ical_datetime(event.end_at)}\r\nCLASS:PUBLIC\r\nDESCRIPTION:100周年イベント2日目です！\r\nGEO:#{event.geo}\r\nLOCATION:〒410-0801 静岡県沼津市大手町1丁目1-4\r\nSUMMARY:市政100周年イベント day.2\r\nEND:VEVENT\r\nEND:VCALENDAR\r\n"
    travel_to(now) do
      assert { calendar.to_ical == expects }
    end
  end

  private

  def format_ical_datetime(datetime)
    datetime.iso8601.gsub('-', '').gsub(':', '').gsub('Z', '')
  end
end
