Rails.application.routes.draw do
  get "up" => "rails/health#show", as: :rails_health_check

  namespace :public do
    get 'health/alive', to: 'health#alive'
    namespace :calendar do
      get :recent
      get :download, params: :id
    end
  end

  namespace :admin do
    namespace :calendar do
      resources :details
      resources :events
    end
  end
end
