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
      docker run --name $container -p 127.0.0.1:8080:8080 $image -a stdin -a stdout -a stderr
    },
    'stop' | 's' => {
      docker stop $container
      docker container rm  $container
    }
    'prune' | 'p' => {
      docker image rm --force $image
    },
    'prune' | 'p' => {
      docker image rm --force $image
    },
    'migrate' | 'm' => {
      psql -d $image -a -f ./src/db/schema.sql
      psql -d $image -a -f ./src/db/mock.sql
    }
    _ => { 
      echo './go.nu [(b)uild | (r)un]'
    }
  }
}
