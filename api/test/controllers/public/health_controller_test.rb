require 'test_helper'

class Public::HealthControllerTest < ActionDispatch::IntegrationTest
  test 'should get alive' do
    get public_health_alive_url
    assert_response :success
  end
end
