require 'test_helper'

class Admin::Calendar::DetailsControllerTest < ActionDispatch::IntegrationTest
  setup do
    @calendar = create(:calendar_detail)
  end

  test 'should get index' do
    get admin_calendar_details_url, as: :json
    assert_response :success
  end

  test 'should create calendar_detail' do
    id = 'numazu-city'
    name = '沼津市イベント情報'
    assert_difference('Calendar::Detail.count') do
      post admin_calendar_details_url, params: { calendar_detail: { id: id, name: name } }, as: :json
    end

    assert_response :created
    calendar = Calendar::Detail.order(:created_at).last
    assert { calendar.id == id }
    assert { calendar.name == name }
  end

  test 'should show calendar_detail' do
    get admin_calendar_detail_url(@calendar), as: :json
    assert_response :success
  end

  test 'should update calendar_detail' do
    new_name = '新・沼津市イベント情報'
    patch admin_calendar_detail_url(@calendar), params: { calendar_detail: { name: new_name } }, as: :json
    assert_response :success
    @calendar.reload
    assert { @calendar.name == new_name }
  end

  test 'should destroy calendar_detail' do
    now = Time.zone.now
    travel_to(now) do
      delete admin_calendar_detail_url(@calendar), as: :json
      assert_response :no_content
      @calendar.reload
      assert { @calendar.discarded_at.to_i == now.to_i }
    end
  end
end
