class Admin::Calendar::EventsController < Admin::Calendar::BaseController
  before_action :set_calendar_event, only: %i[show update destroy]

  def index
    @events = Calendar::Event.kept

    render json: @events
  end

  def show
    render json: @event
  end

  def create
    @event = Calendar::Event.new(calendar_event_params)
    @event.calendar_detail_id = params[:calendar_event][:calendar_id]
    # TODO: いい感じのヘッダー名に変更する
    @event.last_modified_user = request.headers['x-user-name']

    if @event.save
      render json: @event, status: :created, location: admin_calendar_event_url(@event)
    else
      render json: @event.errors, status: :unprocessable_entity
    end
  end

  def update
    # TODO: いい感じのヘッダー名に変更する
    @event.last_modified_user = request.headers['x-user-name']

    if @event.update(calendar_event_params)
      render json: @event
    else
      render json: @event.errors, status: :unprocessable_entity
    end
  end

  def destroy
    @event.discard!
  end

  private

  def set_calendar_event
    @event = Calendar::Event.kept.find(params[:id])
  end

  def calendar_event_params
    params.require(:calendar_event).permit(
      :summary, :description, :location, :latitude, :longitude,
      :start_at, :end_at, :discarded_at
    )
  end
end
