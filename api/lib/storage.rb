class Storage
  attr_reader :client, :bucket_name

  def initialize(bucket_name:, region: nil, access_key_id: nil, secret_access_key: nil, endpoint: nil)
    @client = build_client(region, access_key_id, secret_access_key, endpoint)
    @bucket_name = bucket_name
  end

  def upload(file:, object_key: nil)
    if object_key.present?
      client.put_object(bucket: bucket_name, body: file, key: object_key)
    else
      client.put_object(bucket: bucket_name, body: file, key: file.path)
    end
  end

  private

  def build_client(region, access_key_id, secret_access_key, endpoint)
    Aws::S3::Client.new(
      region: region || ENV.fetch('STORAGE_REGION'),
      access_key_id: access_key_id || ENV.fetch('STORAGE_ACCESS_KEY_ID'),
      secret_access_key: secret_access_key || ENV.fetch('STORAGE_SECRET_ACCESS_KEY'),
      endpoint: endpoint || ENV.fetch('STORAGE_ENDPOINT'),
      force_path_style: true
    )
  end
end
