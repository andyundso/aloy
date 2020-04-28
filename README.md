# Aloy

[![Build Status](https://travis-ci.org/andyundso/aloy.svg?branch=master)](https://travis-ci.org/andyundso/aloy)

Aloy hunts for machines (watching for files) and passes its loot (meta data) to a merchant (webservice).

(Horizon: Zero Dawn is actually a good game).

## Windows build

The windows version of this package currently only builds against Nightly because the package relies on an unstable feature for gathering a file identifier.

## Configuration

Aloy expects a configuration file in the project's root (TBC) as a JSON with an array of the paths you want to watch. An example file is present in the project.

## Future features

* Aloy can handle delete events
* The config file for Aloy can be set through an environment variable
* Aloy can sent a bunch of metadata or only paths to the webservice
* Aloy can log some stuff based on the operating system
  * Windows: Event Logs
  * Linux: Some obscure path under /var/log
  * Mac OS X: ?
* Improve error handling over the whole application
