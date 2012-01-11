require 'dm-core'
require 'dm-types'
require 'dm-timestamps'

configure :development do
    DataMapper::Logger.new(STDOUT, :debug)
end

DataMapper.setup(:default, {
    :adapter    => @db_config[@dbms][RACK_ENV]['adapter'],
    :host       => @db_config[@dbms][RACK_ENV]['host'],
    :username   => @db_config[@dbms][RACK_ENV]['username'],
    :password   => @db_config[@dbms][RACK_ENV]['password'],
    :database   => @db_config[@dbms][RACK_ENV]['database']
})

require ::File.dirname(__FILE__) + '/asset'
