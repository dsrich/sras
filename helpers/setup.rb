RACK_ENV = ENV['RACK_ENV'] || 'test'
@dbms = ENV['DBMS'] || 'mysql'
@db_config = YAML.load_file(::File.dirname(__FILE__) + '/../config/database.yml')
