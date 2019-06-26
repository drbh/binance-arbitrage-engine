


## EC2 Instance Install


```bash
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


```