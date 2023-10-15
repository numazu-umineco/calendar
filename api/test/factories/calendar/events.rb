# == Schema Information
#
# Table name: calendar_events
#
#  id                 :integer          not null, primary key
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
#  calendar_detail_id  (calendar_detail_id => calendar_details.id)
#
FactoryBot.define do
  factory :calendar_event, class: 'Calendar::Event' do
    summary { '市政100周年イベント day.2' }
    description { '100周年イベント2日目です！' }
    location { '〒410-0801 静岡県沼津市大手町1丁目1-4' }
    latitude { 1.5 }
    longitude { 1.5 }
    start_at { 1.day.since }
    end_at { 3.days.since }
    discarded_at {}
    calendar { create(:calendar_detail) }
    last_modified_user { 'umineco-admin' }
  end
end
