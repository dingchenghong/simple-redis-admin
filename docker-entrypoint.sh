#!/bin/bash
sentinel_host=${SENTINEL_HOST}
service_name=${SENTINEL_SERVICE_NAME}
port=${PORT}
if [[ -z $port ]];then
	# default port
    port=8080
fi
/app/redis-admin -p ${port} -n ${service_name} -s ${sentinel_host}
