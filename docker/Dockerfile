FROM sxend/ubuntu:16.04.4

MAINTAINER sxend

RUN cd /opt && wget http://download.redis.io/releases/redis-4.0.1.tar.gz && \
    tar xzf redis-4.0.1.tar.gz && cd redis-4.0.1 && make && make install

RUN apt-get update -y

EXPOSE 3000

ADD ./target /opt/apps/target