def storage
  bucket_name = ENV.fetch('STORAGE_BUCKET_NAME')
  @storage ||= Storage.new(bucket_name: bucket_name)
end

def upload(dir)
  Dir.glob(dir) do |path|
    file = File.open(path)
    filename = File.basename(path)
    storage.upload(file: file, object_key: filename)
  end
end

namespace :upload do
  task storage: :environment do
    Rake::Task['export:ical'].invoke
    Rake::Task['export:json'].invoke
    upload('tmp/export/*.json')
    upload('tmp/export/*.ics')
  end
end
