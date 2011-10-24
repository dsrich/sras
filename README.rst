SRAS (v2) -- Simple Ruby Asset Server... for OpenSimulator
==========================================================

[ Note: I rewrote all of the original code from 2010, so this is
actually a new repo.  If you had cloned the old repo from here, delete
and re-clone to get the latest. ]


Features
--------

 * Asset de-duplication
 * Compressed asset storage on disk
 * Ability to disable serving of specific assets without deleting


Upcoming Features
-----------------

 * MongoDB support


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

If using Thin, just: ::

    $ ./runme.sh

for a single instance.  In a production environment you would want to
run several instances behind a reverse proxy like Nginx.
