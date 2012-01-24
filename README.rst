SRAS -- An Asset Server for OpenSimulator
=========================================

Features
--------

 * Asset de-duplication
 * Compressed asset storage on disk
 * Ability to disable serving of specific assets without deleting


Install
-------

The packages of Ruby shipping with Ubuntu don't include built-in zlib
support.  I prefer using RVM [ http://rvm.beginrescueend.com/ ]
anyway.

Built & testing with Ruby v1.9.2.  Tested with Thin, but should also
work with Passenger.

Install via: ::

    $ gem install sras

then copy the following text into ``/etc/sras/sras.conf`` or
``~/.srasrc`` and edit as appropriate: ::

    sras:
        production:
            default_asset_dir: /srv/sras
            port: 8003
            log_file: /var/log/sras.log
            pid_file: /tmp/sras.pid

    mysql:
        production:
            adapter: mysql
            host: localhost
            username: sras
            password: sras
            database: sras


Running
-------

Then just: ::

    $ sras start

to start a single instance.  In a production environment you would
likely want to run several instances behind a reverse proxy like
Nginx.

To see additional options: ::

    $ sras --help

