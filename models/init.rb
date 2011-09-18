require 'dm-core'
require 'dm-types'
require 'dm-timestamps'

configure :development do
    DataMapper::Logger.new(STDOUT, :debug)
end

DataMapper.setup(:default, {
    :adapter    => 'mysql',
    :host       => 'localhost',
    :username   => 'sras',
    :password   => 'sras',
    :database   => 'sras'
})

require ::File.dirname(__FILE__) + '/asset'
