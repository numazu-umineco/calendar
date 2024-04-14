require 'fileutils'

namespace :export do
  task ical: :environment do
    export_dir = 'tmp/export'
    FileUtils.mkdir_p(export_dir)
    Calendar::Detail.kept.each do |calendar|
      events = calendar.events.where(start_at: [3.months.ago..1.year.since])
      cal = Icalendar::Calendar.new
      events.each do |event|
        event.register!(cal)
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
      events = calendar.events.where(start_at: [3.months.ago..1.year.since])
      File.open("#{export_dir}/#{calendar.id}.json", 'w') do |f|
        f.puts events.to_json
      end
    end
  end
end
