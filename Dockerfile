# Dockerfile

# base image
FROM node:16-bullseye

# create & set working directory
RUN mkdir -p /home/pptruser/app
WORKDIR /home/pptruser/app

# copy source files
COPY . /home/pptruser/app

# install dependencies
RUN npm install

RUN apt-get update \
  && apt-get install -y wget gnupg \
  && wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
  && sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' \
  && apt-get update \
  && apt-get install -y google-chrome-stable fonts-ipafont-gothic fonts-wqy-zenhei fonts-thai-tlwg fonts-kacst fonts-freefont-ttf libxss1 libxshmfence1 \
  --no-install-recommends \
  && rm -rf /var/lib/apt/lists/*

RUN wget https://github.com/digitalocean/doctl/releases/download/v1.70.0/doctl-1.70.0-linux-amd64.tar.gz
RUN tar xf ./doctl-1.70.0-linux-amd64.tar.gz
RUN mv ./doctl /usr/local/bin

# Install puppeteer so it's available in the container.
# Add user so we don't need --no-sandbox.
# same layer as npm install to keep re-chowned files from using up several hundred MBs more space
RUN groupadd -r pptruser && useradd -r -g pptruser -G audio,video pptruser \
  && mkdir -p /home/pptruser/Downloads \
  && chown -R pptruser:pptruser /home/pptruser \
  && chown -R pptruser:pptruser ./node_modules \
  && chown -R pptruser:pptruser ./package.json \
  && chown -R pptruser:pptruser ./package-lock.json

# Run everything after as non-privileged user.
USER pptruser

# start app
EXPOSE 3000
# CMD npm run dev