#!/usr/bin/env ruby

require 'sinatra'
require 'erb'
require 'base64'

require './helpers'

get '/' do
    erb :index
end

get '/assets/:asset_id' do
    fetch_asset(:asset_id)
end

get '/assets/:asset_id/data' do
    fetch_asset(:asset_id)
end

get '/assets/:asset_id/metadata' do
    fetch_asset(:asset_id)
end

post '/assets/?' do
end

delete '/*' do
    # we don't trust anyone to delete assets.  opensim complains if it
    # gets 403s, so return a 404 instead...
    404
end
