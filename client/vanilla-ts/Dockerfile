FROM node:lts AS development

ENV CI=true
ENV PORT=3000

WORKDIR /code
COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json
COPY vite.config.ts /code/vite.config.ts
RUN npm ci
COPY . /code
EXPOSE 3000
##USER node
CMD [ "npm", "run dev" ]