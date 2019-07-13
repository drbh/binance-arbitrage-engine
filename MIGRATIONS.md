# Migration Instructions

make sure postgres is running
```bash
sudo service postgresql start
```

```
export DATABASE_URL=postgres://postgres:$mysecretpassword@localhost
export DATABASE_URL=postgres://drbh2:@localhost
```


```
diesel migration run
```


```
diesel migration redo
```

cant find lds!
```bash

sudo yum install tmux -y

export GOROOT=/usr/local/go
export PATH=$GOROOT/bin:$PATH

export DATABASE_URL=postgres://postgres:$mysecretpassword@localhost
export LD_LIBRARY_PATH=/usr/local/lib
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig/



#/usr/local/lib/libzmq.so
```