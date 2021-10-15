#!/bin/bash

docker exec -it postgres sh -c "PGPASSWORD=docker /usr/bin/psql -h 127.0.0.1 -U postgres -d postgres"
