FROM debian:11.1-slim

# mysql versionを固定
ARG MYSQL_VERSION=5.7.26

# mysql clientのインストール
RUN mkdir /tmp/lib
WORKDIR  /tmp/lib

RUN wget https://downloads.mysql.com/archives/get/file/libmysqlclient20_$MYSQL_VERSION-1debian9_amd64.deb  && \
    wget https://downloads.mysql.com/archives/get/file/libmysqlclient-dev_$MYSQL_VERSION-1debian9_amd64.deb && \
    wget https://downloads.mysql.com/archives/get/file/libmysqlclient20-dbgsym_$MYSQL_VERSION-1debian9_amd64.deb && \
    wget https://downloads.mysql.com/archives/get/file/mysql-community-client_$MYSQL_VERSION-1debian9_amd64.deb && \
    wget https://downloads.mysql.com/archives/get/file/mysql-community-client-dbgsym_$MYSQL_VERSION-1debian9_amd64.deb

RUN dpkg -i libmysqlclient20_$MYSQL_VERSION-1debian9_amd64.deb  && \
    dpkg -i libmysqlclient-dev_$MYSQL_VERSION-1debian9_amd64.deb && \
    dpkg -i libmysqlclient20-dbgsym_$MYSQL_VERSION-1debian9_amd64.deb && \
    dpkg -i mysql-community-client_$MYSQL_VERSION-1debian9_amd64.deb && \
    dpkg -i mysql-community-client-dbgsym_$MYSQL_VERSION-1debian9_amd64.deb

RUN mysql --version

CMD ["/bin/sh", "-c", "mysql -uadmin -hmoco-moco-cluster-01.moco-cluster.svc.cluster.local -pmypwds < /app/fulldb-11-12-2021-15-57-beta.sql"]