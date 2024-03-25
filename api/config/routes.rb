Rails.application.routes.draw do
  get 'up' => 'rails/health#show', as: :rails_health_check
  get 'public/healthcheck', to: 'healthcheck#show'
  get 'admin/healthcheck', to: 'healthcheck#show'

  namespace :public do
    namespace :calendar do
      get :all
      get 'download/:id', action: :download

      namespace :event do
        get :recent
      end
    end
  end

  namespace :admin do
    namespace :calendar do
      resources :details
      resources :events
    end
  end
end
