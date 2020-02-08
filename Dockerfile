FROM rustlang/rust:nightly

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY . /usr/src/app

CMD ["cargo", "build", "-q"]