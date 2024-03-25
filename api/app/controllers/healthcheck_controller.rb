class HealthcheckController < ApplicationController
  def show
    ActiveRecord::Base.connection.execute('SELECT 1')
    render json: { status: 'ok' }
  end
end
