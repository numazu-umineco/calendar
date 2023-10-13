# == Schema Information
#
# Table name: calendar_events
#
#  id                 :bigint           not null, primary key
#  description        :text             not null
#  discarded_at       :datetime
#  end_at             :datetime         not null
#  last_modified_user :string           default(""), not null
#  latitude           :float
#  location           :text
#  longitude          :float
#  start_at           :datetime         not null
#  summary            :text             not null
#  created_at         :datetime         not null
#  updated_at         :datetime         not null
#  calendar_detail_id :string           not null
#
# Indexes
#
#  index_calendar_events_on_calendar_detail_id  (calendar_detail_id)
#  index_calendar_events_on_discarded_at        (discarded_at)
#
# Foreign Keys
#
#  fk_rails_...  (calendar_detail_id => calendar_details.id)
#
require 'test_helper'

class Calendar::EventTest < ActiveSupport::TestCase
  subject { build(:calendar_event) }
  should validate_presence_of(:summary)
  should validate_presence_of(:description)
  should validate_presence_of(:start_at)
  should validate_presence_of(:end_at)
  should belong_to(:calendar)

  test '#geoは緯度経度をフォーマットした文字列を返す' do
    latitude = 35.123
    longitude = 77.0369
    event = create(:calendar_event, latitude: latitude, longitude: longitude)
    assert { event.geo == "#{latitude};#{longitude}" }
  end

  test '#register!は受け取ったcalendarオブジェクトにイベントを登録していく' do
    calendar = Icalendar::Calendar.new
    event = create(:calendar_event, latitude: 35.123, longitude: 138.987)
    assert { calendar.events.empty? }

    uid = SecureRandom.uuid
    event.stubs(:gen_uuid).returns(uid)
    event.register!(calendar)
    assert { calendar.events.length == 1 }
    assert { calendar.events[0].summary == event.summary }
    assert { calendar.events[0].description == event.description }
    assert { calendar.events[0].location == event.location }
    assert { calendar.events[0].geo == event.geo }
    assert { calendar.events[0].dtstart.to_i == event.start_at.to_i }
    assert { calendar.events[0].dtend.to_i == event.end_at.to_i }
  end

  test '.recentは最新のイベントを指定した個数取得する' do
    events = Array.new(5) { create(:calendar_event) }
    assert { Calendar::Event.recent(3) == events.reverse.first(3) }
  end
end
