FROM certbot/certbot:latest
WORKDIR /etc/letsencrypt/live/localhost
RUN \
  openssl req -x509 -out fullchain.pem -keyout privkey.pem \
    -newkey rsa:2048 -nodes -sha256 \
    -subj '/CN=localhost' -days 1