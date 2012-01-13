root = ::File.dirname(::File.realdirpath(__FILE__))
require "#{root}/app.rb"
run SRAS.new
