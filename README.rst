SRAS -- Simple Ruby Asset Server... for OpenSimulator
==========================================================

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

Built & testing with Ruby v1.9.2.

Install gems: ::

    $ gem install bundler
    $ cd /path/to/sras
    $ bundle install

Tested with Thin, but should also work with Passenger.


Running
-------

Configure: ::

    $ cp config/config.yml.example config/config.yml
    $ # edit config/config.yml to your liking
    $ rake db:create

Then, if using Thin, just: ::

    $ ./runme.sh

for a single instance.  In a production environment you would want to
run several instances behind a reverse proxy like Nginx.
