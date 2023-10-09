class Public::HealthController < ApplicationController
  def alive
    render json: { status: 'ok' }
  end
end
