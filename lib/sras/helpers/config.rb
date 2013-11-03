#
# load config file
#

def load_config_file(config_file=nil)
    if config_file.nil?
        if File.exists?("#{ROOT}/config/config.yml")
            config_file = "#{ROOT}/config/config.yml"
        elsif File.exists?(File.expand_path('~/.srasrc'))
            config_file = File.expand_path('~/.srasrc')
        elsif File.exists?('/etc/sras/sras.conf')
            config_file = '/etc/sras/sras.conf'
        else
            warn "Couldn't find SRAS config file.  Exiting."
            exit 1
        end
    end

    begin
        return YAML.load_file(config_file) 
    rescue
        abort "Oops.  Couldn't open config file #{config_file} ...  Aborting."
    end
end
