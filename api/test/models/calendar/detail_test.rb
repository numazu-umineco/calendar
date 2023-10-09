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

module Calendar
  class DetailTest < ActiveSupport::TestCase
    should validate_presence_of(:name)
  end
end
