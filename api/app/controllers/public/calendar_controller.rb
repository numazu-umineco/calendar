class Public::CalendarController < ApplicationController
  def all
    calendars = Calendar::Detail.kept
    render json: calendars
  end

  def download
    calendar = Calendar::Detail.kept.find(params[:id])
    send_data calendar.to_ical, filename: "#{calendar.name}.ics"
  end
end
