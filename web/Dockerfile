FROM node:alpine

# install simple http server for serving static content
RUN npm install -g http-server

# make the 'app' folder the current working directory
WORKDIR /app

# copy project files and folders to the current working directory (i.e. 'app' folder)
COPY . .

# install dependencies and build app for production with minification
RUN npm install
RUN npm run build

EXPOSE 8080
CMD [ "http-server", "dist" ]
