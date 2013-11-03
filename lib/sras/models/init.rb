require 'dm-core'
require 'dm-types'
require 'dm-timestamps'

module SRAS
    if ENV['RACK_ENV'] == 'development'
        DataMapper::Logger.new(STDOUT, :debug)
    end

    DataMapper.setup(:default, {
        :adapter    => 'mysql',
        :host       => Config.db_host,
        :username   => Config.username,
        :password   => Config.password,
        :database   => Config.database
    })
end

require ::File.dirname(__FILE__) + '/asset'
