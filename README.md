# `no-more-json`

Fed up with JSON APIs? Just want one little value from a massive JSON object? Don't want to
parse the JSON yourself?

`no-more-json` does all this for you! Here's an example:

Before 😓:

```
curl https://httpbin.org/ip
{
  "origin": "102.222.181.247"
}
```

After 🥳:

```
curl 'https://jq-ify.shuttleapp.rs/api?url=https://httpbin.org/ip&q=.origin'
"13.41.13.143"
```

Let's break that down:

`  https://jq-ify.shuttleapp.rs/api  <-- This is the API for no-more-json
     ?url=https://httpbin.org/ip    <-- This is where you specify the url for your endpoint
     &q=.origin                     <-- This is where you put your jq query to parse the JSON`
If you don't know about `jq`, check this out: https://jqlang.github.io/jq/

So this lets you parse arbitrary JSON blobs and just get back a simple scalar value, without
having to parse the JSON yourself, include a json-parsing library, or trying to include the
`jq` binary into your service.

Here's a more complicated example:

<table align="center">
  <tr>
    <th align="center">
       Before
    </th>
    <th align="center">
        After
    </th>
  </tr>
  <tr>
    <td align="center">

```
code
```

    </td>
    <td align="center">

```
code

```

    </td>

  </tr>
</table>
