FROM node:16-bullseye

RUN mkdir -p /home/node/app/node_modules && chown -R node:node /home/node/app

WORKDIR /home/node/app

# Install dependencies for puppeteer and imagemagick
RUN apt-get update \
  && apt-get install libfuse2 fuse \
  && apt-get install -y wget gnupg \
  && wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
  && sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' \
  && apt-get update \
  && apt-get install -y google-chrome-stable fonts-ipafont-gothic fonts-wqy-zenhei fonts-thai-tlwg fonts-kacst fonts-freefont-ttf libxss1 libxshmfence1 \
  --no-install-recommends \
  && rm -rf /var/lib/apt/lists/*

# Install imagemagick
RUN wget https://download.imagemagick.org/ImageMagick/download/binaries/magick
RUN chmod 777 ./magick
RUN chown node:node ./magick

# Install digital ocean cli tool
RUN wget https://github.com/digitalocean/doctl/releases/download/v1.70.0/doctl-1.70.0-linux-amd64.tar.gz
RUN tar xf ./doctl-1.70.0-linux-amd64.tar.gz
RUN mv ./doctl /usr/local/bin

COPY --chown=node:node package*.json ./

USER node

RUN npm install

COPY --chown=node:node . .

CMD [ "npm", "run", "update-photos" ]