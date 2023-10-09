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
FactoryBot.define do
  factory :calendar_detail, class: 'Calendar::Detail' do
    id { SecureRandom.hex }
    name { 'MyString' }
  end
end
