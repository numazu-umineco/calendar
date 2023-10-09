Rails.application.routes.draw do
  get "up" => "rails/health#show", as: :rails_health_check

  namespace :public do
    get 'health/alive', to: 'health#alive'
  end

  namespace :admin do
    namespace :calendar do
      resources :details
    end
  end
end
