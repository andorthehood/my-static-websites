[![Netlify Status](https://api.netlify.com/api/v1/badges/a8bd44af-89f0-4afe-8765-f9cfc38191bf/deploy-status)](https://app.netlify.com/sites/andor/deploys)

# andor.cool

Hello there! 👋 You just found the source code of my personal website. It's powered by a static site generator I wrote in Rust, without using any third-party libraries. This whole project has been a personal playground for me to improve my, let’s say, “rusty” Rust skills. (I wanted to write this in C, but Netlify still doesn't support deploying C projects.) 

Originally, my site was built with Jekyll, because I liked how easy it was to use Markdown files with Front Matter and Liquid templates for my posts and pages. Even after trying out Next.js and then moving to Rust, I've kept this approach to manage my content.

## Requirements

To run the website locally, make sure you have the following installed:
- Rust: The project is developed in Rust, so you'll need the Rust compiler and Cargo (its package manager) installed on your system.

## Installation

To get the website running on your local machine, follow these steps:

    git clone https://github.com/hngrhorace/andor.cool.git
    cd andor.cool
    cargo run
