# rust_cimport

This is a work-in-progress port of cimport to Rust, and a pilot exercise in porting the CAIRIS server side code to the language.

To run it, you will need to pass the following arguments:  db host, db port, db user, db password, db name, and model file.

## Running on a non-CAIRIS server

The original cimport.py script was designed to run on the same machine as the CAIRIS server.  It is possible to run rust_cimport on some other machine but, to do this, the MySQL server needs to allow remote mysql connections.  You can set this up by setting `bind-address = 0.0.0.0` in `mysqld.cnf` (usually found in `/etc/mysql/mysql.conf.d` in Ubuntu).

## Running the tests

You will need to run a [CAIRIS test database](https://github.com/failys/cairis_test_db) to run all the tests.

To run the tests, you need to create a .env file in the root directory with the below values.  The only thing you'll want to change is the value of `RESET_SERVER`, which will vary depending on where you put this.

```
export DB_HOST=localhost
export DB_PORT=3306
export DB_USER=cairis_test
export DB_PASSWD=cairis_test
export TEST_DB=cairis_test_default
export MODEL_FILE=tests/test.xml
export RESET_SERVER=/Users/sfaily/cairis_test_db/resetServer.sh
```