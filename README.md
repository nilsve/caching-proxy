# Caching proxy server

A simple proxy server with (currently in-memory only) caching.
The idea is that when you are for frontend work for example, instead of connecting directly to the API, you configure your browser to use this application as a HTTP proxy server. It will cache any request it sees, thus making api calls that can normally take seconds now respond instantly.

Written in Rust, it's still a WIP.

