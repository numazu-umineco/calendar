require 'test_helper'

class Admin::Calendar::EventsControllerTest < ActionDispatch::IntegrationTest
  setup do
    @calendar = create(:calendar_detail)
    @event = create(:calendar_event, calendar: @calendar)
  end

  test 'should get index' do
    get admin_calendar_events_url, as: :json
    assert_response :success
  end

  test 'should create calendar_event' do
    summary = '沼津夏まつり・狩野川花火大会'
    description = '第76回沼津夏まつり・狩野川花火大会を2023年7月29日㈯、30日㈰に開催します。'
    location = '静岡県沼津市'
    start_at = DateTime.parse('2023-7-29 17:00')
    end_at = DateTime.parse('2023-7-30 20:00')
    latitude = 35.095584
    longitude = 138.859776
    modified_user = 'umineco-admin'

    assert_difference('Calendar::Event.count') do
      post admin_calendar_events_url,
           headers: { 'x-user-name': modified_user },
           params: {
             calendar_event: {
               calendar_id: @calendar.id,
               summary: summary, description: description, location: location,
               start_at: start_at, end_at: end_at,
               latitude: latitude, longitude: longitude
             }
           },
           as: :json
    end
    assert_response :created
    event = Calendar::Event.last
    assert { event.summary == summary }
    assert { event.description == description }
    assert { event.start_at.to_i == start_at.to_i }
    assert { event.end_at.to_i == end_at.to_i }
    assert { event.latitude == latitude }
    assert { event.longitude == longitude }
    assert { event.last_modified_user == modified_user }
  end

  test 'should show calendar_event' do
    get admin_calendar_event_url(@event), as: :json
    assert_response :success
  end

  test 'should update calendar_event' do
    summary = '沼津夏まつり・狩野川花火大会'
    description = '第76回沼津夏まつり・狩野川花火大会を2023年7月29日㈯、30日㈰に開催します。'
    location = '静岡県沼津市'
    start_at = DateTime.parse('2023-7-29 17:00')
    end_at = DateTime.parse('2023-7-30 20:00')
    latitude = 35.095584
    longitude = 138.859776
    modified_user = 'umineco-staff'
    patch admin_calendar_event_url(@event),
          headers: { 'x-user-name': modified_user },
          params: {
            calendar_event: {
              summary: summary, description: description, location: location,
              start_at: start_at, end_at: end_at,
              latitude: latitude, longitude: longitude
            }
          },
          as: :json
    assert_response :success
    @event.reload
    assert { @event.summary == summary }
    assert { @event.description == description }
    assert { @event.start_at.to_i == start_at.to_i }
    assert { @event.end_at.to_i == end_at.to_i }
    assert { @event.latitude == latitude }
    assert { @event.longitude == longitude }
    assert { @event.last_modified_user == modified_user }
  end

  test 'should destroy calendar_event' do
    now = Time.zone.now
    travel_to(now) do
      delete admin_calendar_event_url(@event), as: :json
      assert_response :no_content
      @event.reload
      assert { @event.discarded_at.to_i == now.to_i }
    end
  end
end
