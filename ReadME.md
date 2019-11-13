# Data Cannon Core
Data Cannon contains the shared libraries for the client and worker in this distributed
job queue.

This framework is like Celery but for Rust and uses asyncio. Celery v2.0
message protcol is used.

More will follow...

## See

For more information related to job queues:

https://docs.celeryproject.org/en/latest/index.html

## Features

Existing features included in release 0.1:

    - SSL support
    - AMQP/RabbitMQ broker support
    - Elastic Search backend Support
    - Redis backend support
    - RPC backend support
    - Client and Workers
    - Routing Key Support
    - Registry support
    - Message protocol support and serialization
    - Identification matching Celery
    - Threadable connections in worker ;)
    - Tokio support in the clinet ;)
    - Working on Kafka Support ;)
    
Features to include later (0.2+):

    - Creation of a messaging framework like Kombu
    - All other backends
    - OAuth2.0 support (RabbitMQ, Elasticsearch)
    - monitoring support (PRIORITY)
    - healthcecking support (PRIORITY)
    - Rust implemented LevelDB Broker
    - Upgrade Functions
    - 1 to 1 feature matching with celery and maybe some extras

Sorry guys, I am one man on a very specific mission. All thigns considered v0.2+ is 6 months off.

## License

Copyright 2019- Andrew Evans

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.