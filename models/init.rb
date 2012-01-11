require 'dm-core'
require 'dm-types'
require 'dm-timestamps'

configure :development do
    DataMapper::Logger.new(STDOUT, :debug)
end

DataMapper.setup(:default, {
    :adapter    => @config[@dbms][RACK_ENV]['adapter'],
    :host       => @config[@dbms][RACK_ENV]['host'],
    :username   => @config[@dbms][RACK_ENV]['username'],
    :password   => @config[@dbms][RACK_ENV]['password'],
    :database   => @config[@dbms][RACK_ENV]['database']
})

require ::File.dirname(__FILE__) + '/asset'
