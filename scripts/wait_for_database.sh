#!/bin/bash

cd ..
docker-compose up -d

until pg_isready -h localhost -p 5432 -U postgres
do 
  echo "Waiting for postges"
  sleep 2;
done

echo "docker is now running"
docker-compose down
