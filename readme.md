This Repository contains the code for a HTTP server as there are no HTTP server comes with standard library.
The <mark>http</mark> directory contains the library code for the HTTP server.
The <mark>httpserver</mark> directory contains binary code for the Http server.

The server runs on 3000 port.

Use the following command to run the server
```cargo run -p httpserver```

These are the endpoints supported by the server
```localhost:3000/```

```localhost:3000/health```

```localhost:3000/api/shipping/orders```

```localhost:3000/invalid-path```