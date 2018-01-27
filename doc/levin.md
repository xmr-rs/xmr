# levin is not that mysterious

The Monero protocol is based on levin, a tcp protocol used to communicate with
other nodes. It acts as both a server and a client.

Here are the fundamental concepts in levin:

1. **Command:** a command used to talk to another peers, commands are identified
by an unsigned integer (`uint32_t`), this usually starts at 1.000
(e:g: `1000 + 1`).

2. **Invoke:** used to craft commands that need a response, you can think of it
as a method, for example you can call a command called `ADD` with to integers
as the input and receive the result.

3. **Notify:** used for commands that don't need a response, you can create
a `HELLO` command without receiving a response from the peer.

And that's basically all.

## Going inside

Here I'm going to talk about the low-level structures used in levin to
communicated between peers.

Invoke and notify commands both start with a header called `bucket_head`
this stores information to read the tcp stream contents. It has this definition:

```c++
struct bucket_head {
    uint64_t m_signature;
    uint64_t m_cb;
    bool     m_have_to_return_data;
    uint32_t m_command;
    int32_t  m_return_code;
    uint32_t m_reservedA; //probably some flags in future
    uint32_t m_reservedB; //probably some check sum in future
}
```

Have you noted the `m_reservedA` and `m_reservedB` and the comments on the left?
well this is because this definition is a bit old and a new one is used (this
is what the Monero protocol uses):

```c++
struct bucket_head2
{
    uint64_t m_signature;
    uint64_t m_cb;
    bool     m_have_to_return_data;
    uint32_t m_command;
    int32_t  m_return_code;
    uint32_t m_flags;
    uint32_t m_protocol_version;
};
```

Ok, so we have now all the necessary information so let's explain it clearly
field by field:

1. `m_signature`: just used to identify this is a levin `bucket_head` and not
other type of packet. It should match this definition:

```c++
#define LEVIN_SIGNATURE  0x0101010101012101LL  //Bender's nightmare
```

(Also, what does "Bende'rs nightmare" means? if you know please share it with
me, just curious).

2. `m_cb`: The size of the data after this header, it doesn't include the size
of `bucket_head`.

3. `m_have_to_return_data`: this value is `true` for invoke commands and false
for notify commands.

4. `m_command`: The ID of the command.

5. `m_return_code`: If it's a response a return code is provided. It should
match one of:

```c++
#define LEVIN_OK                                        0
#define LEVIN_ERROR_CONNECTION                         -1
#define LEVIN_ERROR_CONNECTION_NOT_FOUND               -2
#define LEVIN_ERROR_CONNECTION_DESTROYED               -3
#define LEVIN_ERROR_CONNECTION_TIMEDOUT                -4
#define LEVIN_ERROR_CONNECTION_NO_DUPLEX_PROTOCOL      -5
#define LEVIN_ERROR_CONNECTION_HANDLER_NOT_DEFINED     -6
#define LEVIN_ERROR_FORMAT                             -7
```

6. `m_flags`: Some flags about this command, here are ones:

```c++
#define LEVIN_PACKET_REQUEST		0x00000001
#define LEVIN_PACKET_RESPONSE		0x00000002
```

7. `m_protocol_version`: The version used currently. It should match one of
the followings:

```c++
#define LEVIN_PROTOCOL_VER_0         0
#define LEVIN_PROTOCOL_VER_1         1			
```

However, Monero just accepts `LEVIN_PROTOCOL_VER_1`.

## Command format

Ok we've seen a lot of tcp handling stuff but.. How are commands parsed? Well
Monero uses format for it's commands that is a bit esoteric and not much elegant
but you know it just works and I can live with that. That format is called 
**Portable Storage**.

This thing called Portable Storage can serialize complex data structures. Some
fundamental concepts of it:

1. **Section:** they are used to represent structures, it's just a *Hash Map*
with a string as the key (the name of the field in the structure.), and a 
*Storage Entry* as the value.

2. **Storage Entry:** they store the information of a single field in a
structure. It can be any signed/unsigned integer, array, buffer, or another
section.

### Binary representation

TODO: Explain this boring thing.
