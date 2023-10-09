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
class Calendar::Detail < ApplicationRecord
  include Discard::Model

  validates :name, presence: true, uniqueness: true

  has_many :events,
           class_name: 'Calendar::Event',
           foreign_key: :calendar_detail_id,
           inverse_of: :calendar

  def to_ical
    cal = Icalendar::Calendar.new
    events.each do |event|
      event.register!(cal)
    end
    cal.publish
    cal.to_ical
  end
end
