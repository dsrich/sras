require ::File.dirname(__FILE__) + '/lib/sras/helpers/version.rb'

Gem::Specification.new do |s|
    s.name          = 'sras'
    s.version       = Version
    s.summary       = 'Simple Ruby Asset Server'
    s.description   = 'Asset server for use with OpenSimulator'
    s.authors       = 'Dave Coyle'
    s.email         = 'hello@coyled.com'
    s.homepage      = 'http://coyled.com/sras'
    s.license       = 'MIT'

    s.required_ruby_version = '>= 1.9.2'
    s.files         = `git ls-files`.split("\n")
    s.test_files    = `git ls-files -- test/*`.split("\n")
    s.executables   = `git ls-files -- bin/*`.split("\n").map { |f| File.basename(f) }

    s.add_runtime_dependency    'dm-migrations', '1.2.0'
    s.add_runtime_dependency    'dm-mysql-adapter', '1.2.0'
    s.add_runtime_dependency    'dm-timestamps', '1.2.0'
    s.add_runtime_dependency    'dm-types', '1.2.2'
    s.add_runtime_dependency    'hpricot', '0.8.6'
    s.add_runtime_dependency    'sinatra', '1.4.3'
    s.add_runtime_dependency    'thin', '1.6.1'
    s.add_runtime_dependency    'rake', '10.1.0'
end
