require 'dm-core'
require 'dm-types'

DataMapper::Logger.new(STDOUT, :debug)
DataMapper.setup(:default, {
    :adapter    => 'mysql',
    :host       => 'localhost',
    :username   => 'sras',
    :password   => 'sras',
    :database   => 'sras'
})

require ::File.dirname(__FILE__) + '/asset'
