##### Very much a work-in-progess. The idea will remain, but the endpoint will likely change (for the better).

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
curl 'https://no-more-json.shuttleapp.rs/api?url=https://httpbin.org/ip&q=.origin'
"13.41.13.143"
```

Let's break that down:

```
https://no-more-json.shuttleapp.rs/api  <-- This is the API for no-more-json
  ?url=https://httpbin.org/ip     <-- This is where you specify the url for your endpoint
  &q=.origin                      <-- This is where you put your jq query to parse the JSON
```
If you don't know about `jq`, check this out: https://jqlang.github.io/jq/

So this lets you parse arbitrary JSON blobs and just get back a simple scalar value, without
having to parse the JSON yourself, include a json-parsing library, or trying to include the
`jq` binary into your service.

Here's a more complicated example:

Before (using data from wttr.in):

```
curl 'wttr.in/Detroit?format=j1'
{
    "current_condition": [
        {
            "FeelsLikeC": "27",     <-- The thing we want
            29 other lines we don't want ):
        }
    ],
    "nearest_area": [...],          <-- 25 lines we don't want ):
    "request": [...],               <-- 4 lines we don't want 
    "weather": [...]                <-- 1248 lines we don't want!
}
```
The jq query we'd use to extract the feels-like temperature in Celcius is `.current_condition[0].FeelsLikeC`

We can use `no-more-json` to make a request to the wttr.in endpoint, extract
the result, and then run the jq query for us on that result, so we just get
back a nice scalar value:

```
curl 'https://no-more-json.shuttleapp.rs/api?url=https://wttr.in/Detroit?format=j1&q=.current_condition%5B0%5D.FeelsLikeC'
"27"
```

(The URL has been encoded so that the special characters in the jq query (like `[`, `]`) aren't treated specially.)

Isn't that cool!
