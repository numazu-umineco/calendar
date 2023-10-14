class Admin::Calendar::DetailsController < Admin::Calendar::BaseController
  before_action :set_calendar_detail, only: %i[show update destroy]

  def index
    @calendar_details = Calendar::Detail.kept

    render json: @calendar_details
  end

  def show
    render json: @calendar_detail
  end

  def create
    @calendar_detail = Calendar::Detail.new(calendar_detail_params)

    if @calendar_detail.save
      render json: @calendar_detail, status: :created, location: admin_calendar_detail_url(@calendar_detail)
    else
      render json: @calendar_detail.errors, status: :unprocessable_entity
    end
  end

  def update
    if @calendar_detail.update(calendar_detail_params)
      render json: @calendar_detail
    else
      render json: @calendar_detail.errors, status: :unprocessable_entity
    end
  end

  def destroy
    @calendar_detail.discard!
  end

  private

  def set_calendar_detail
    @calendar_detail = Calendar::Detail.kept.find(params[:id])
  end

  def calendar_detail_params
    params.require(:calendar_detail).permit(:id, :name)
  end
end
