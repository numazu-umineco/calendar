class Admin::Calendar::BaseController < ApplicationController
  before_action :verify_header

  private

  def verify_header
    # NOTE: ヘッダーの情報を検証する処理を書いても良いかも
  end
end
