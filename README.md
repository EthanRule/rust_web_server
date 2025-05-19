# Rust-Web-Server

There are two different servers. The first server is implemented using the Hyper crate. And the second
server is implemented based off the `Rust Programming Book` with additional features and error handling.  

The hyper server was partially built first while the `Rust Programming Book` server is more extensive
and is the current version in continual development.

## Hyper Server

### Support

- HTTP/2
- IPV4, IPV6
- GET

### Send Client -> Server Requests
```curl.exe -X POST http://localhost:3000/echo/reversed -d "Some data"```  
```curl.exe http://localhost:3000/```  

## Server (`Rust Programming Book` extension)

### Support

- HTTP/1
- Multithreading
- GET

#### Sending Requests
Visit: `127.0.0.1:7878` or `127.0.0.1:7878/sleep`

Sleep requests take 5 seconds to respond. The server allows multiple  
requests to be sent at a time, using a threadpool and queue to manage.  

Threadpool size is determined by the number of threads your CPU's hardware  
has available.  

