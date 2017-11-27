+++
date        = "2016-03-25"
title       = "Why and when you should use SoA"
tags        = [ "D", "soa", "dod" ]
+++


# What is SoA?

SoA just means `Structure of arrays` 

```d
//AoS: Array of structures
struct Vec2{
    float x;
    float y;
}
Array!Vec2 vectors;
```
```d
//SoA: Structure of arrays
struct Vec2{
    float[] x;
    float[] y;
}
```

# Why is SoA useful?

Imagine you want to write a small `udp game server` with a `client server architecture`. You will have a server where clients can connect to. The `server` needs to remember which clients are currently connected. The server polls messages with `recvfrom` and in case you are not familiar with udp `recvfrom` returns the packet that was sent to the port where to socket was bound to and the address.

When a packet comes in the first thing you probably want to know is if the packet came from a connected client. You would be inclined to write it like this:

```c++
struct Server{
    struct RemoteClient{
        Address address;
        SysTime lastReceivedPacket;
        //more data
    }
    Array!RemoteClient remoteClients;

    void poll(){
        //Address address
        //recvfrom(buffer, address);
    }
}
```


If we want to know which client has sent the package we can just use the `remoteClients` array to find the correct `remoteClient`. The problem is that we need to iterate over `RemoteClient` but we are only really interested in the address field. That means we needlessly load all the other data like `lastReceivedPacket` even if we don't need it.

And if you are curious how much data could be inside a `RemoteClient` in a real world application, here is the struct of [Enet Peer](https://github.com/lsalzman/enet/blob/master/include/enet/enet.h#L258). It might not be the fairest comparison because it is a `Peer` and not a `RemoteClient` but it should illustrate the point that your data might grow fairly large.

```c
typedef struct _ENetPeer
{ 
   ENetListNode  dispatchList;
   struct _ENetHost * host;
   enet_uint16   outgoingPeerID;
   enet_uint16   incomingPeerID;
   enet_uint32   connectID;
   enet_uint8    outgoingSessionID;
   enet_uint8    incomingSessionID;
   ENetAddress   address;            /**< Internet address of the peer */
   void *        data;               /**< Application private data, may be freely modified */
   ENetPeerState state;
   ENetChannel * channels;
   size_t        channelCount;       /**< Number of channels allocated for communication with peer */
   enet_uint32   incomingBandwidth;  /**< Downstream bandwidth of the client in bytes/second */
   enet_uint32   outgoingBandwidth;  /**< Upstream bandwidth of the client in bytes/second */
   enet_uint32   incomingBandwidthThrottleEpoch;
   enet_uint32   outgoingBandwidthThrottleEpoch;
   enet_uint32   incomingDataTotal;
   enet_uint32   outgoingDataTotal;
   enet_uint32   lastSendTime;
   enet_uint32   lastReceiveTime;
   enet_uint32   nextTimeout;
   enet_uint32   earliestTimeout;
   enet_uint32   packetLossEpoch;
   enet_uint32   packetsSent;
   enet_uint32   packetsLost;
   enet_uint32   packetLoss;          /**< mean packet loss of reliable packets as a ratio with respect to the constant ENET_PEER_PACKET_LOSS_SCALE */
   enet_uint32   packetLossVariance;
   enet_uint32   packetThrottle;
   enet_uint32   packetThrottleLimit;
   enet_uint32   packetThrottleCounter;
   enet_uint32   packetThrottleEpoch;
   enet_uint32   packetThrottleAcceleration;
   enet_uint32   packetThrottleDeceleration;
   enet_uint32   packetThrottleInterval;
   enet_uint32   pingInterval;
   enet_uint32   timeoutLimit;
   enet_uint32   timeoutMinimum;
   enet_uint32   timeoutMaximum;
   enet_uint32   lastRoundTripTime;
   enet_uint32   lowestRoundTripTime;
   enet_uint32   lastRoundTripTimeVariance;
   enet_uint32   highestRoundTripTimeVariance;
   enet_uint32   roundTripTime;
   enet_uint32   roundTripTimeVariance;
   enet_uint32   mtu;
   enet_uint32   windowSize;
   enet_uint32   reliableDataInTransit;
   enet_uint16   outgoingReliableSequenceNumber;
   ENetList      acknowledgements;
   ENetList      sentReliableCommands;
   ENetList      sentUnreliableCommands;
   ENetList      outgoingReliableCommands;
   ENetList      outgoingUnreliableCommands;
   ENetList      dispatchedCommands;
   int           needsDispatch;
   enet_uint16   incomingUnsequencedGroup;
   enet_uint16   outgoingUnsequencedGroup;
   enet_uint32   unsequencedWindow [ENET_PEER_UNSEQUENCED_WINDOW_SIZE / 32]; 
   enet_uint32   eventData;
   size_t        totalWaitingData;
} ENetPeer;
```

Now let us see how it would look with `SoA`.

```c++
struct Server{
    struct RemoteClients{
        size_t length;
        Address[] address;
        SysTime[] lastReceivedPacket;
        //more data
    }
    RemoteClients remoteClients;

    void poll(){
        //Address address
        //recvfrom(buffer, address);
    }
}
```

We can access all addresses with `remoteClients.address` and we don't need to load unnecessary data into the cache.

# Isn't SoA awkward to use?

In most languages it is.

```c++
struct RemoteClients{
    size_t length;
    Address[] address;
    SysTime[] lastReceivedPacket;
    //more data
}
```

The definition is simplified because we need to allocate the arrays, grow them if we want to have dynamic arrays. We also need to worry about inserting and removing elements, it shouldn't happen that we only add an address to `RemoteClients` without adding `lastReceivedPacket`. That is because the data is loosely coupled. Previously with `AoS` we could access the `RemoteClient` with `remoteClients[index]` but now we are accessing a `RemoteClient` by its components
`remoteClients.addresses[index]` and `remoteClients.lastReceivedPacket[index]`.


# Implementing SoA in D

First let us start with a demonstration.
```d
struct Vec2{
    float x;
    float y;
}
auto s = SOA!(Vec2)();

s.insertBack(1.0f, 2.0f);
s.insertBack(Vec2(1.0, 2.0f));
writeln(s.x); // [1, 1]
writeln(s.y); // [2, 2]
```

We can still create a struct with our data, `SOA` will then look at the struct and create the correct arrays internally. `insertBack` is now a bit different from a normal array because internally we have as many arrays as we have fields. That means `insertBack` needs to be variadic. Alternatively `insertBack` could also accept the struct itself.


_The following code is not intended to be production ready code, it is merely a proof of concept._

```d
struct SOA(T){
    import std.experimental.allocator;
    import std.experimental.allocator.mallocator;

    import std.meta: staticMap;
    import std.typecons: Tuple;
    import std.traits: FieldNameTuple;

    alias toArray(T) = T[];
    alias toType(string s) = typeof(__traits(getMember, T, s));

    alias MemberNames = FieldNameTuple!T;
    alias Types = staticMap!(toType, MemberNames);
    alias ArrayTypes = staticMap!(toArray, Types);

    this(size_t _size, IAllocator _alloc = allocatorObject(Mallocator.instance)){
        alloc = _alloc;
        size = _size;
        allocate(size);
    }

    ref auto opDispatch(string name)(){
        import std.meta: staticIndexOf;
        alias index = staticIndexOf!(name, MemberNames);
        static assert(index >= 0);
        return containers[index];
    }

    void insertBack(Types types){
        if(length == size) grow;
        foreach(index, ref container; containers){
            container[length] = types[index];
        }
        length = length + 1;
    }

    void insertBack(T t){
        if(length == size) grow;
        foreach(index, _; Types){
            containers[index][length] = __traits(getMember, t, MemberNames[index]);
        }
        length = length + 1;
    }

    size_t length() const @property{
        return _length;
    }

    ~this(){
        if(alloc is null) return;
        foreach(ref container; containers){
            alloc.dispose(container);
        }
    }

private:
    void length(size_t len)@property{
        _length = len;
    }

    Tuple!ArrayTypes containers;
    IAllocator alloc;

    size_t _length = 0;
    size_t size = 0;
    short growFactor = 2;

    void allocate(size_t size){
        if(alloc is null){
            alloc = allocatorObject(Mallocator.instance);
        }
        foreach(index, ref container; containers){
            container = alloc.makeArray!(Types[index])(size);
        }
    }

    void grow(){
        import std.algorithm: max;
        size_t newSize = max(1,size * growFactor);
        size_t expandSize = newSize - size;

        if(size is 0){
            allocate(newSize);
        }
        else{
            foreach(ref container; containers){
                alloc.expandArray(container, expandSize);
            }
        }
        size = newSize;
    }
}
```

```d
alias toArray(T) = T[];
alias toType(string s) = typeof(__traits(getMember, T, s));

alias MemberNames = FieldNameTuple!T;
alias Types = staticMap!(toType, MemberNames);
alias ArrayTypes = staticMap!(toArray, Types);
```

`MemberNames` are just the names of the fields. For example `struct Vec2{float x; float y}` will have the type `AliasSeq!("x", "y")`. `toType ` takes the member name and turns it into an actual type. In the example above `toType!("x")` would return the type `float`.


Now we can convert the member names into actual types with the help of `staticMap`. In the example above `AliasSeq!("x", "y")` would be transformed into `AliasSeq!(float, float)`.

We are almost there we just now need to convert the types to arrays. `AliasSeq!(float, float)` to `AliasSeq!(float[], float[])`. We do this with `toArray` and `staticMap`

After that we can create a tuple of arrays

```d
Tuple!ArrayTypes containers;
```

Inserting elements is fairly easy now.
```d
void insertBack()(Types types){
    if(length == size) grow;
    foreach(index, ref container; containers){
        container[length] = types[index];
    }
    length = length + 1;
}
```

We already now what types `insertBack` should accept. It should accept the types of the fields of the struct. We then iterate over `containers` at compile time, which is our tuple of arrays.

Then we just access the correct `argument` with `types[index]` and insert it into the array.

We can also insert the struct itself.
```d
void insertBack(T t){
    if(length == size) grow;
    foreach(index, _; Types){
        containers[index][length] = __traits(getMember, t, MemberNames[index]);
    }
    length = length + 1;
}
```

We iterate over the Types to get the `index`. We use `index` to get the correct container and to find the correct field name of the struct. This works because the order is always the same.

The code above for `Vec2` would roughly look like this

```d
void insertBack(Vec2 t){
    if(length == size) grow;
    containers[0][length] = t.x;
    containers[1][length] = t.y;
    length = length + 1;
}
```

We can access the arrays with the field names. In D this is very easy to do with `opDispatch`.

```d
ref auto opDispatch(string name)(){
    import std.meta: staticIndexOf;
    alias index = staticIndexOf!(name, MemberNames);
    static assert(index >= 0);
    return containers[index];
}
```

In the example above for `Vec2` we can get to the array of all x's with `s.x` or all y's with 's.y'. `opDispatch` would roughly look like this at compile time if we call `s.x`.

```d
ref auto opDispatch(){
    import std.meta: staticIndexOf;
    alias index = staticIndexOf!("x", MemberNames);
    static assert(index >= 0);
    return containers[index];
}
```

We just get the index of `opDispatch` `name` in `MemberNames`, if it is not inside `MemberNames` `opDispatch` will fail. If it is inside `MemberNames` we just access the correct container with the index.


```c++
struct Server{
    struct RemoteClients{
        Address address;
        SysTime lastReceivedPacket;
        //more data
    }
    SOA!RemoteClient remoteClients;

    void poll(){
        //Address address
        //recvfrom(buffer, address);
    }
}
```


# When to use SoA

Firs ot all `SoA` is not a silver bullet and it doesn't mean you should replace `AoS` with `SoA` everywhere in your code base.

`SoA` makes sense if:

* You know that you want to store your data in an array.
* You want partial access to the data.

But sometimes you still want to access all components of your data. An example would be a vector.

```d
struct Vec3{
    float x;
    float y;
    float z;
}
```

Most operations will use all components anyways like add, subtract, dot, length and many more. And even if you sometimes end up with

```d
struct Vec3{
    float x;
    float y;
    float z;
}

Array!Vec3 positions;

positions[].filter!(v => v.x < 10.0f);
```

and you want to filter all vectors where the `x component` is less than `10.0f`, you  will still only load two additional floats. Also a `Vec3` struct won't get bigger in time, other data structures might grow and become a bottleneck in the future.


# Isn't SoA premature optimization?

In my opinion it is not. The problem with `AoS` is that if it becomes a performance bottleneck in the future, you will have to refactor a lot of code. For example you might want to pack your data into a struct hot and cold like this:

```d
struct Bar{
    struct Hot{
        Data1 d1;
        Data2 d2;
        ...
    }
    struct Cold{
        Data3 d3;
        Data4 d4;
        ...
    }

    Hot* hot;
    Cold* cold;
}
```

but depending on the language you will still have to refactor a lot of code. It might save you some trouble to think about your data access early on.

[Jonathan Blow](https://www.youtube.com/watch?v=ZHqFrNyLlpA) has a language demonstration that covers SoA and anonymous variables.
_Quick note: `Jonathan Blow`'s `using` is very similar to `alias this` in D._

`SoA` isn't much worse compared to `AoS` depending on the language you use.

```d
//AoS
remoteClients[index].address;

//vs 

//SoA
remoteClients.address[index];

```

But `SoA` scales much better because you can partially access your data without needlessly loading unrelevant data into your cache.

