require 'fileutils'

namespace :export do
  task ical: :environment do
    export_dir = 'tmp/export'
    FileUtils.mkdir_p(export_dir)
    Calendar::Detail.kept.each do |calendar|
      File.open("#{export_dir}/#{calendar.name}.ics", 'w') do |f|
        f.puts calendar.to_ical
      end
    end
  end

  task json: :environment do
    export_dir = 'tmp/export'
    FileUtils.mkdir_p(export_dir)
    Calendar::Detail.kept.each do |calendar|
      events = calendar.events.where(start_at: [1.month.ago..2.months.since])
      File.open("#{export_dir}/#{calendar.name}.json", 'w') do |f|
        f.puts events.to_json
      end
    end
  end
end
