# localserver
Simple server to serve specified folder locally

## Motivation

Sometimes you just need a simple server that would serve a certain directory. For example, when you're compiling your code to Web Assembly, browser won't let you access those files via file URI scheme (```file:\\```), you'll need to serve them via http.

## Implementation

It leverages the power of [warp](https://github.com/seanmonstar/warp), so all the complexity is handled there.

## Usage

localserver </path/to/folder> <port> (optional)
