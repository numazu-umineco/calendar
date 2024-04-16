require 'fileutils'

namespace :export do
  task ical: :environment do
    export_dir = 'tmp/export'
    FileUtils.mkdir_p(export_dir)
    Calendar::Detail.kept.each do |calendar|
      events = calendar.events.where(start_at: [3.months.ago..1.year.since]).order(:start_at)
      cal = Icalendar::Calendar.new
      events.each do |event|
        if event.end_at - events.start_at > 4.days
          event_start = event.dup
          event_start.title = "#{event.title} (開始▶)"
          event_start.start_at = event.start_at
          event_start.end_at = event.start_at
          event.register!(cal)
          event_end = event.dup
          event_end.title = "#{event.title} (◀終了)"
          event_end.start_at = event.end_at
          event_end.end_at = event.end_at
          event.register!(cal)
        else
          event.register!(cal)
        end
      end
      cal.publish
      File.open("#{export_dir}/#{calendar.id}.ics", 'w') do |f|
        f.puts cal.to_ical
      end
    end
  end

  task json: :environment do
    export_dir = 'tmp/export'
    FileUtils.mkdir_p(export_dir)
    Calendar::Detail.kept.each do |calendar|
      events = calendar.events.where(start_at: [3.months.ago..1.year.since]).order(:start_at)
      File.open("#{export_dir}/#{calendar.id}.json", 'w') do |f|
        f.puts events.to_json
      end
    end
  end
end
