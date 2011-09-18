#!/bin/sh

/usr/bin/env thin -e production -p 8003 -R config.ru start
