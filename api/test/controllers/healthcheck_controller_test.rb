require 'test_helper'

class HealthcheckControllerTest < ActionDispatch::IntegrationTest
  test 'should get `/public/healthcheck' do
    get public_healthcheck_url
    assert_response :success
  end

  test 'should get `/admin/healthcheck' do
    get admin_healthcheck_url
    assert_response :success
  end
end
