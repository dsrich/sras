module SRAS
    class Version
        MAJOR = '2'
        MINOR = '1'
        PATCH = '6'

        class << self
            def to_s
               [MAJOR, MINOR, PATCH].compact.join('.')
            end
        end
    end
end
