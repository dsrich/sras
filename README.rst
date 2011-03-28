SRAS (v2) -- Simple Ruby Asset Server... for OpenSimulator
==========================================================

[ Note: I rewrote all of the original code, so this is actually a new
repo.  If you had cloned the old repo from here, delete and re-clone
to get the latest. ]


Status
------

Still have some small bits to complete and some more testing to do in
the next few days.

*** This version isn't ready for production use yet. ***

    -- coyled, 2011-03-27


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

Install gems:

    # gem install dm-mysql-adapter dm-types hpricot sinatra

Tested with Thin, but should also work with Passenger.  So:

    # gem install thin

or:

    # gem install passenger


Running
-------

If using Thin, just:

    $ ./runme.sh

for a single instance.
