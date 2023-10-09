ENV['RAILS_ENV'] ||= 'test'
require_relative '../config/environment'
require 'rails/test_help'
require 'minitest/reporters'

module ActiveSupport
  class TestCase
    parallelize(workers: :number_of_processors)

    include FactoryBot::Syntax::Methods
  end
end

Shoulda::Matchers.configure do |config|
  config.integrate do |with|
    with.test_framework :minitest
    with.library :rails
  end
end
