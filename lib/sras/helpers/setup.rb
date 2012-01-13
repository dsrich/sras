RACK_ENV = ENV['RACK_ENV'] || 'production'
@dbms = ENV['DBMS'] || 'mysql'
@config = YAML.load_file("#{ROOT}/config/config.yml")
