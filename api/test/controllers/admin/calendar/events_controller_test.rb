require 'test_helper'

# rubocop:disable Metrics/ClassLength
class Admin::Calendar::EventsControllerTest < ActionDispatch::IntegrationTest
  setup do
    @calendar = create(:calendar_detail)
  end

  test 'イベント情報を返す(start_at,end_atを指定しない場合は今日から30日後までの間に開催されるものが返される)' do
    now = Time.zone.now
    events = (1..4).map do |i|
      create(:calendar_event, calendar: @calendar, start_at: (9 * i).days.since(now))
    end
    get admin_calendar_events_url
    json = parse_body(body)
    assert { json.size == 3 }
    json[0].each do |key, value|
      assert { same?(events[0].__send__(key), value) }
    end
    json[1].each do |key, value|
      assert { same?(events[1].__send__(key), value) }
    end
    json[2].each do |key, value|
      assert { same?(events[2].__send__(key), value) }
    end
  end

  test '期間を指定してイベントを取得できる' do
    now = Time.zone.today
    events = (1..4).map do |i|
      create(:calendar_event, calendar: @calendar, start_at: (9 * i).days.since(now))
    end

    travel_to(now) do
      get admin_calendar_events_url, params: { start_at: now, end_at: 8.days.since(now) }
      json = parse_body(body)
      assert { json.blank? }
    end

    travel_to(now) do
      get admin_calendar_events_url, params: { start_at: now, end_at: 9.days.since(now) }
      json = parse_body(body)
      assert { json.size == 1 }
      json[0].each do |key, value|
        assert { same?(events[0].__send__(key), value) }
      end
    end

    travel_to(now) do
      get admin_calendar_events_url, params: { start_at: now, end_at: 17.days.since(now) }
      json = parse_body(body)
      assert { json.size == 1 }
      json[0].each do |key, value|
        assert { same?(events[0].__send__(key), value) }
      end
    end

    travel_to(now) do
      get admin_calendar_events_url, params: { start_at: now, end_at: 18.days.since(now) }
      json = parse_body(body)
      assert { json.size == 2 }
      json[0].each do |key, value|
        assert { same?(events[0].__send__(key), value) }
      end
      json[1].each do |key, value|
        assert { same?(events[1].__send__(key), value) }
      end
    end

    travel_to(now) do
      get admin_calendar_events_url, params: { start_at: now, end_at: 36.days.since(now) }
      json = parse_body(body)
      assert { json.size == 4 }
      json[0].each do |key, value|
        assert { same?(events[0].__send__(key), value) }
      end
      json[1].each do |key, value|
        assert { same?(events[1].__send__(key), value) }
      end
      json[2].each do |key, value|
        assert { same?(events[2].__send__(key), value) }
      end
      json[3].each do |key, value|
        assert { same?(events[3].__send__(key), value) }
      end
    end
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
    event = create(:calendar_event, calendar: @calendar)
    get admin_calendar_event_url(event), as: :json
    assert_response :success
  end

  test 'should update calendar_event' do
    event = create(:calendar_event, calendar: @calendar)
    summary = '沼津夏まつり・狩野川花火大会'
    description = '第76回沼津夏まつり・狩野川花火大会を2023年7月29日㈯、30日㈰に開催します。'
    location = '静岡県沼津市'
    start_at = DateTime.parse('2023-7-29 17:00')
    end_at = DateTime.parse('2023-7-30 20:00')
    latitude = 35.095584
    longitude = 138.859776
    modified_user = 'umineco-staff'
    patch admin_calendar_event_url(event),
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
    event.reload
    assert { event.summary == summary }
    assert { event.description == description }
    assert { event.start_at.to_i == start_at.to_i }
    assert { event.end_at.to_i == end_at.to_i }
    assert { event.latitude == latitude }
    assert { event.longitude == longitude }
    assert { event.last_modified_user == modified_user }
  end

  test 'should destroy calendar_event' do
    event = create(:calendar_event, calendar: @calendar)
    now = Time.zone.now
    travel_to(now) do
      delete admin_calendar_event_url(event), as: :json
      assert_response :no_content
      event.reload
      assert { event.discarded_at.to_i == now.to_i }
    end
  end

  private

  def same?(expects, actual)
    case expects
    when DateTime, ActiveSupport::TimeWithZone
      expects.to_i == DateTime.parse(actual).to_i
    when Date
      expects.to_i == Date.parse(actual).to_i
    else
      expects == actual
    end
  end

  def parse_body(body)
    JSON.parse(body, symbolize_names: true)
  end
end
# rubocop:enable Metrics/ClassLength
