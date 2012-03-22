def config_file()
    if File.exists?("#{ROOT}/config/config.yml")
        return "#{ROOT}/config/config.yml"
    elsif File.exists?(File.expand_path('~/.srasrc'))
        return File.expand_path('~/.srasrc')
    else
        return '/etc/sras/sras.conf'
    end
end
