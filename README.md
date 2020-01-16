[![Netlify Status](https://api.netlify.com/api/v1/badges/a8bd44af-89f0-4afe-8765-f9cfc38191bf/deploy-status)](https://app.netlify.com/sites/andor/deploys)

# andor.cool
Source code of my photoblog
<http://andor.cool>

## Requirements
- Linux, Unix or macOS
- Ruby v2 or later
- RubyGems
- Bundler

If you don't have Bundler, you can install it by typing:

    gem install bundler

## Installation

    git clone https://github.com/hngrhorace/andor.cool.git
    cd andor.cool
    git submodule update --init --recursive --remote
    bundle install
    bundle exec jekyll serve
