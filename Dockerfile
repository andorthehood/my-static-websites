# Dockerfile

# base image
FROM node:16-bullseye

# create & set working directory
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# copy source files
COPY . /usr/src/app

# install dependencies
RUN npm install

RUN wget https://github.com/digitalocean/doctl/releases/download/v1.70.0/doctl-1.70.0-linux-amd64.tar.gz
RUN tar xf ./doctl-1.70.0-linux-amd64.tar.gz
RUN mv ./doctl /usr/local/bin

# start app
EXPOSE 3000
CMD npm run dev