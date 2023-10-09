Rails.application.routes.draw do
  get "up" => "rails/health#show", as: :rails_health_check

  namespace :public do
    get 'health/alive', to: 'health#alive'
  end
end
