# mongo-testing

investigating a bug relating to mongoimport / mongoexport tool on local mongodb using docker running on windows 10.


Set up testing environment for developing mongodb tools with active server & mock data to work on.
Assumes that development system is on MacOS.

* Ensure that homebrew is installed. Install git, docker, docker-compose & mongosh.
* Clone this repo
* Run container for mongodb: `docker-compose -f mongo-compose.yml`

This should suffice for testing on the server for any mongo-tools related testing.

TODOs
- [] write mocker in rust (must be able to generate document size of 4GB / 8GB / 16GB / 32GB)
- [] must be able to delete data quickly and efficiently (look up async operations in mongodb, tokio async driver?)
- [] find build instructions from mongo-tools (integration with the repo)
- [] write measurement tool using rust to call the built go tools (can use bash scripts / rust for this)

