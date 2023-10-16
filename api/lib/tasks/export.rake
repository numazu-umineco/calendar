require 'fileutils'

namespace :export do
  task all: :environment do
    export_dir = 'tmp/export'
    FileUtils.mkdir_p(export_dir)
    Calendar::Detail.kept.each do |calendar|
      File.open("#{export_dir}/#{calendar.name}.ics", 'w') do |f|
        f.puts calendar.to_ical
      end
    end
  end
end
