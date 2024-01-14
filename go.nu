#!/bin/nu

let image = 'sample-site'
let tag = 'latest'
let container = $"($image)-container"

def main [x?: string] {
  match $x {
    'build' | 'b' => { 
      docker build --tag $"($image):($tag)" .
    },
    'run' | 'r' => {
      docker run --init --name $container -p 127.0.0.1:8080:8080 $image -a stdin -a stdout -a stderr
    },
    _ => { 
      echo './go.nu [(b)uild | (r)un]'
    }
  }
}
