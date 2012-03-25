#
# load config file
#

def load_config_file(config_file)
    unless File.exists?(config_file)
        if File.exists?("#{ROOT}/config/config.yml")
            config_file = "#{ROOT}/config/config.yml"
        elsif File.exists?(File.expand_path('~/.srasrc'))
            config_file = File.expand_path('~/.srasrc')
        elsif File.exists('/etc/sras/sras.conf')
            config_file = '/etc/sras/sras.conf'
        else
            warn "Couldn't find SRAS config file.  Exiting."
            exit 1
        end
    end

    return YAML.load_file(config_file)
end
