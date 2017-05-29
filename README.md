# InfluxDB test

## Setup

Start influxdb container

```
docker run -p 8086:8086 \
      -v /tmp/influxdb:/var/lib/influxdb \
      influxdb
```

Create database

```
curl -XPOST 'http://localhost:8086/query' --data-urlencode "q=CREATE DATABASE testing"
```

## Insert data

```
cargo build
./target/debug/influxdb_test "http://localhost:8086/write?db=testing"
```

## Count data

```bash
curl -G "http://localhost:8086/query?pretty=true" --data-urlencode "db=testing" \
--data-urlencode "q=SELECT count(amount) FROM pay"
```
