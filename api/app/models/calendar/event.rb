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
class Calendar::Event < ApplicationRecord
  include Discard::Model

  validates :summary, presence: true
  validates :description, presence: true
  validates :start_at, presence: true
  validates :end_at, presence: true

  belongs_to :calendar,
             class_name: 'Calendar::Detail',
             foreign_key: :calendar_detail_id,
             inverse_of: :events

  scope :recent, ->(limit) { order(created_at: :desc).limit(limit) }

  def geo
    "#{latitude};#{longitude}"
  end

  # rubocop:disable Metrics/AbcSize
  def register!(cal)
    cal.event do |e|
      e.uid = gen_uuid
      e.dtstart = Icalendar::Values::DateTime.new(start_at)
      e.dtend = Icalendar::Values::DateTime.new(end_at)
      e.summary = summary
      e.description = description
      e.location = location
      e.geo = [latitude, longitude]
      e.ip_class = 'PUBLIC'
    end
  end
  # rubocop:enable Metrics/AbcSize

  private

  def gen_uuid
    SecureRandom.uuid
  end
end
