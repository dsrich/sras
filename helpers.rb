helpers do
    def fetch_asset(asset_id)

    end

    def flags
        # Flags added to opensim in 9b22393.  Current values (from
        # OpenSim/Framework/AssetBase.cs):
        #
        #   [Flags]
        #   public enum AssetFlags : int
        #   {
        #       Normal = 0,
        #       Maptile = 1,
        #       Rewritable = 2,
        #       Collectable = 4
        #   }
        'Normal'
    end

    def content_type
        if self.asset_type == 0
            'image/jp2'
        elsif self.asset_type == 1
            'application/ogg'
        elsif self.asset_type == 2
            'application/x-metaverse-callingcard'
        elsif self.asset_type == 3
            'application/x-metaverse-landmark'
        elsif self.asset_type == 5
            'application/x-metaverse-clothing'
        elsif self.asset_type == 6
            'application/x-metaverse-primitive'
        elsif self.asset_type == 7
            'application/x-metaverse-notecard'
        elsif self.asset_type == 8
            'application/x-metaverse-folder'
        elsif self.asset_type == 10
            'application/x-metaverse-lsl'
        elsif self.asset_type == 11
            'application/x-metaverse-lso'
        elsif self.asset_type == 12
            'image/tga'
        elsif self.asset_type == 13
            'application/x-metaverse-bodypart'
        elsif self.asset_type == 17
            'audio/x-wav'
        elsif self.asset_type == 19
            'image/jpeg'
        elsif self.asset_type == 20
            'application/x-metaverse-animation'
        elsif self.asset_type == 21
            'application/x-metaverse-gesture'
        elsif self.asset_type == 22
            'application/x-metaverse-simstate'
        else
            'application/octet-stream'
        end
    end
end
