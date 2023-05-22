# mongo-testing

investigating a bug relating to mongoimport / mongoexport tool on local mongodb using docker running on windows 10.


## Install
Set up testing environment for developing mongodb tools with active server & mock data to work on.
Assumes that development system is on MacOS.

* Ensure that homebrew is installed. Install git, docker, docker-compose & mongosh.
* Clone this repo
* Run container for mongodb: `docker-compose -f mongo-compose.yml`

This should suffice for testing on the server for any mongo-tools related testing.

## Run script
The rust package contains the mongodb mocker to test with the tools built from mongotools. This local testing my not be representative of the
actually deploy production environments, but serves as a good test bed for testing the mongo-tools and using the latest mongodb release.

### building
```
> cargo build
```

### running
```bash
> cargo run -- mongodb://localhost test_col 1024
Usage: mongo_mocker <MONGO_URI> <TARGET_COLLECTION> <COLLECTION_SIZE>                                                                                                                                                                                                               Arguments:                                                                                                                                  <MONGO_URI>                                                                                                                               <TARGET_COLLECTION>                                                                                                                       <COLLECTION_SIZE>

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## TODOs
- [x] write mocker in rust (must be able to generate document size of 4GB / 8GB / 16GB / 32GB)
- [] must be able to delete data quickly and efficiently (look up async operations in mongodb, tokio async driver?)
- [] find build instructions from mongo-tools (integration with the repo)
- [] write measurement tool using rust to call the built go tools (can use bash scripts / rust for this)


## References
https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/
https://rust-cli.github.io/book/tutorial/impl-draft.html
https://betterprogramming.pub/persistent-databases-using-dockers-volumes-and-mongodb-9ac284c25b39
https://stackoverflow.com/questions/37366857/how-to-pass-arguments-to-entrypoint-in-docker-compose-yml

