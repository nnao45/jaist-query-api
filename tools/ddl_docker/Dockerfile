FROM imega/mysql-client:10.6.4

RUN mkdir /app
WORKDIR /app
COPY tools/db_data/fulldb-11-12-2021-15-57-beta.sql .

CMD ["/bin/sh", "-c", "mysql -uadmin -hmoco-moco-cluster-01.moco-cluster.svc.cluster.local -pmypwds < /app/fulldb-11-12-2021-15-57-beta.sql"]