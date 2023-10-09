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
require "test_helper"

class Calendar::EventTest < ActiveSupport::TestCase
  subject { build(:calendar_event) }
  should validate_presence_of(:summary)
  should validate_presence_of(:description)
  should validate_presence_of(:start_at)
  should validate_presence_of(:end_at)
  should belong_to(:calendar)
end
