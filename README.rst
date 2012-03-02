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

Then just: ::

    $ bin/sras start

to start a single instance.  In a production environment you would
likely want to run several instances behind a reverse proxy like
Nginx.

To see additional options: ::

    $ bin/sras --help


Testing
-------

To test asset creation: ::

    $ curl -d @test/test.asset -X POST -w '%{http_code}\n' \
        http://yourserver.example.com:8003/assets/

You should get a 200 HTTP response, have an entry in your assets
table, and have a file contain the asset data on disk.

To test retrieval of that asset: ::

   $ curl -X GET \
        http://yourserver.example.com:8003/assets/0193663d-44e4-4e6e-9a1c-8dd2febc5fc5/data 


Mailing List
------------

For SRAS-related announcements and discussion feel free to join the
low-volume mailing list.  To join the list send an email to
majordomo@sdf.org with the following body (no subject required):

    subscribe sras-l

Mailing list members can email the list via sras-l@sdf.org

List archives can be found at http://lists.coyled.com/sras-l/archives/
