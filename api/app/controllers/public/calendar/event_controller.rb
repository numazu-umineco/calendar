class Public::Calendar::EventController < ApplicationController
  def recent
    events = Calendar::Event.kept.recent(ENV.fetch('RECENT_EVENTS_LIMIT', 10))
    render json: events
  end
end
