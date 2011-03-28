require 'dm-core'
require 'dm-types'

DataMapper.setup(:default, {
    :adapter    => 'mysql',
    :host       => 'localhost',
    :username   => 'sras',
    :password   => 'sras',
    :database   => 'sras'
})

DataMapper::Logger.new(STDOUT, :debug)

require ::File.dirname(__FILE__) + '/asset'
