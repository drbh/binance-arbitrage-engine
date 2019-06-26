


## EC2 Instance Install


```bash

## FROM https://gist.github.com/toebbel/6834175230afea9f05c1765fcf783627

# compilers
sudo yum install -y gcc gcc-c++

# libsodium: download, compile, install, remove intermediate files
curl https://download.libsodium.org/libsodium/releases/libsodium-1.0.10.tar.gz | tar -xz
cd libsodium-1.0.11
./configure && make && sudo make install
cd ../ && rm -rf libsodium-1.0.11

# zmq 4.2.0
wget https://github.com/zeromq/libzmq/releases/download/v4.2.0/zeromq-4.2.0.tar.gz 
tar xfz zeromq-4.2.0.tar.gz && rm zeromq-4.2.0.tar.gz
cd zeromq-4.2.0
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig
./configure
make
sudo make install
cd ../ && rm -rf zeromq-4.2.0

# make sure zmq lib can be found
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib


# remove compilers
# sudo yum remove -y gcc-c++ gcc

sudo yum install postgresql postgresql-server postgresql-devel \
postgresql-contrib postgresql-docs -y

# init a new db
sudo service postgresql initdb

### EASY SETUP FROM 

## start db
sudo service postgresql start

sudo su - postgres
psql -U postgres

ALTER USER postgres WITH PASSWORD '$mysecretpassword';

sudo nano /var/lib/pgsql/data/pg_hba.conf
sudo service postgresql restart

##
#https://github.com/diesel-rs/diesel/issues/2026#issuecomment-493648449

# make a bigger swap
sudo dd if=/dev/zero of=/swapfile bs=1M count=2048
sudo chmod 600 /swapfile
ls -l /swapfile 
sudo mkswap /swapfile
sudo swapon /swapfile

swapon -s

sudo nano etc/fstab
/swapfile swap swap defaults 0 0

cargo install diesel_cli

```

```
local   all             postgres                                md5

# IPv4 local connections:
host    all             all             127.0.0.1/32            md5
```