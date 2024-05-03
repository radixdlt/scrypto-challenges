# OpenRPC Server JS

<center>
  <span>
    <img alt="CircleCI branch" src="https://img.shields.io/circleci/project/github/open-rpc/server-js/master.svg">
    <img src="https://codecov.io/gh/open-rpc/server-js/branch/master/graph/badge.svg" />
    <img alt="Dependabot status" src="https://api.dependabot.com/badges/status?host=github&repo=open-rpc/server-js" />
    <img alt="npm" src="https://img.shields.io/npm/dt/@open-rpc/server-js.svg" />
    <img alt="GitHub release" src="https://img.shields.io/github/release/open-rpc/server-js.svg" />
    <img alt="GitHub commits since latest release" src="https://img.shields.io/github/commits-since/open-rpc/server-js/latest.svg" />
  </span>
</center>

JSON-RPC 2.0 + OpenRPC Server implementation that supports multiple transport protocols. Built to run with node 10+.

Need help or have a question? Join us on [Discord](https://discord.gg/gREUKuF)!

## Features

 - [x] Multiple Transports
   - [x] HTTP
   - [x] HTTPS1/2
   - [x] WebSockets
   - [x] IPC
     - [x] UDP
     - [x] TCP
 - [x] Automatically Validate JSON Schemas for [ContentDescriptor#schemas](https://spec.open-rpc.org/#content-descriptor-schema) [MethodObject#params](https://spec.open-rpc.org/#method-result).
 - [x] CLI to start a server by configuration

## How to Use

install server, and optionally schema-utils-js if you want to dereference/validate the open-rpc document before running.
```bash
npm install --save @open-rpc/server-js @open-rpc/schema-utils-js
```

Write an open-rpc document describing your service
`./src/openrpc.json`
see: https://raw.githubusercontent.com/open-rpc/examples/master/service-descriptions/simple-math-openrpc.json
or write your own in [playground](https://playground.open-rpc.org/).


For each of the methods, create a function that has the same name
`src/method-mapping`
```typescript
import { MethodMapping } from "@open-rpc/server-js/build/router";
export const methodMapping: MethodMapping = {
  addition: (a: number, b: number) => a + b,
  subtraction: (a: number, b: number) => a - b
};
export default methodMapping;
```

Create a server with the methods and the document, serve it over http and websocket
`src/server.ts`
```typescript
import { Server, ServerOptions } from "@open-rpc/server-js";
import { HTTPServerTransportOptions } from "@open-rpc/server-js/build/transports/http";
import { WebSocketServerTransportOptions } from "@open-rpc/server-js/build/transports/websocket";
import { OpenrpcDocument } from "@open-rpc/meta-schema";
import { parseOpenRPCDocument } from "@open-rpc/schema-utils-js";
import methodMapping from "./method-mapping";
import doc from "./openrpc.json";

export async function start() {
  const serverOptions: ServerOptions = {
    openrpcDocument: await parseOpenRPCDocument(doc as OpenrpcDocument),
    transportConfigs: [
      {
        type: "HTTPTransport",
        options: {
          port: 3330,
          middleware: [],
        } as HTTPServerTransportOptions,
      },
      {
        type: "WebSocketTransport",
        options: {
          port: 3331,
          middleware: [],
        } as WebSocketServerTransportOptions,
      },
    ],
    methodMapping,
  };

  console.log("Starting Server"); // tslint:disable-line
  const s = new Server(serverOptions);

  s.start();
}
```

#### Lower Level Bits

##### Creating Routers

###### using method mapping and OpenRPC document

```typescript
import { types } from "@open-rpc/meta-schema";
import { Router } from "@open-rpc/server-js";

const openrpcDocument = {
  openrpc: "1.0.0",
  info: {
    title: "node-json-rpc-server example",
    version: "1.0.0"
  },
  methods: [
    {
      name: "addition",
      params: [
        { name: "a", schema: { type: "integer" } },
        { name: "b", schema: { type: "integer" } }
      ],
      result: {
        { name: "c", schema: { type: "integer" } }
      }
    }
  ]
} as types.OpenRPC;

const methodHandlerMapping = {
  addition: (a: number, b: number) => Promise.resolve(a + b)
};

const router = new Router(openrpcDocument, methodHandlerMapping);
```

###### mock mode

```typescript
const router = new Router(openrpcDocument, { mockMode: true });
```

##### Creating Transports

###### IPC

```typescript
import { TCPIPCServerTranport, UDPIPCServerTranport } from "@open-rpc/server-js";

const ipcOptions = { maxConnetions: 20 }; // https://www.npmjs.com/package/node-ipc#ipc-config
const TCPIPCOptions = { ...ipcOptions, networkPort: 4343 };
const UDPIPCOptions = { ...ipcOptions, networkPort: 4343, udp: true };

const tcpIpcTransport = new IPCServerTranport(TCPIPCTransportOptions);
const UdpIpcTransport = new IPCServerTranport(UDPIPCTransportOptions);
```

###### HTTP/S

```
import { HTTPServerTransport, HTTPSServerTransport } from "@open-rpc/server-js";

const httpOptions = {
  middleware: [ cors({ origin: "*" }) ],
  port: 4345
};
const httpsOptions = { // extends https://nodejs.org/api/https.html#https_https_createserver_options_requestlistener
  middleware: [ cors({ origin: "*" }) ],
  port: 4346,
  key: await fs.readFile("test/fixtures/keys/agent2-key.pem"),
  cert: await fs.readFile("test/fixtures/keys/agent2-cert.pem"),
  ca: fs.readFileSync("ssl/ca.crt")
};

const httpTransport = new HTTPServerTransport(httpOptions);
const httpsTransport = new HTTPSServerTransport(httpsOptions); // Defaults to using HTTP2, allows HTTP1.
```

###### WebSockets

```
import { WebSocketServerTransport } from "@open-rpc/server-js";

const webSocketFromHttpsOptions = { // extends https://github.com/websockets/ws/blob/master/doc/ws.md#new-websocketserveroptions-callback
  server: httpsTransport.server
};

const webSocketOptions = { // extends https://github.com/websockets/ws/blob/master/doc/ws.md#new-websocketserveroptions-callback
  port: 4347
};
const wsFromHttpsTransport = new WebSocketServerTransport(webSocketFromHttpsOptions); // Accepts http transport as well.
const wsTransport = new WebSocketServerTransport(webSocketOptions); // Accepts http transport as well.
```

###### Add components as you go
```
const server = new Server();
server.start();

server.addTransport(httpsTransport); // will be started immediately
server.setRouter(router);
server.addTransports([ wsTransport, wsFromHttpsTransport, httpsTransport ]); // will be started immediately.
```

### Contributing

How to contribute, build and release are outlined in [CONTRIBUTING.md](CONTRIBUTING.md), [BUILDING.md](BUILDING.md) and [RELEASING.md](RELEASING.md) respectively. Commits in this repository follow the [CONVENTIONAL_COMMITS.md](CONVENTIONAL_COMMITS.md) specification.
